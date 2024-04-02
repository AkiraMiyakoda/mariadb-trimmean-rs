// Copyright © 2024 Akira Miyakoda
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use rust_decimal::prelude::*;
use udf::prelude::*;

#[derive(Debug)]
pub(crate) struct Trimmean {
    values: Vec<Decimal>,
    proportion: Decimal,
}

#[register]
impl BasicUdf for Trimmean {
    type Returns<'a> = Option<String>;

    fn init(cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
        // Check the number of args
        if args.len() != 2 {
            return Err(format!("Expected 2 args, but got {}", args.len()));
        }

        // Check if the 1st args is numeric
        match args.get(0).unwrap().value() {
            SqlResult::Real(_) | SqlResult::Decimal(_) | SqlResult::Int(_) => {}
            _ => return Err("1st arg must be real, decimal or int".into()),
        }

        // Check if the 2nd arg is numeric and in [0.0, 1.0)
        let proportion = match args.get(1).unwrap().value() {
            SqlResult::Real(Some(value)) => Decimal::from_f64(value),
            SqlResult::Decimal(Some(value)) => Decimal::from_str_exact(value).ok(),
            _ => return Err("2nd arg must be real or decimal".into()),
        };
        let proportion = match proportion {
            Some(prop) => prop,
            None => return Err(String::from("Failed to convert 2nd arg into decimal")),
        };
        if proportion < Decimal::ZERO || Decimal::ONE <= proportion {
            return Err(String::from("2nd arg out of range [0.0, 1.0)"));
        }

        cfg.set_maybe_null(true);

        Ok(Self {
            values: vec![],
            proportion,
        })
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        _args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        if self.values.is_empty() {
            return Ok(None);
        }

        // Calculate the number of elements trimmed
        let len = Decimal::new(self.values.len() as i64, 0);
        let trim = ((len * self.proportion) / Decimal::TWO)
            .floor()
            .to_usize()
            .unwrap();

        // Trim the top and bottom elements
        let values = if trim > 0 {
            self.values.sort_unstable();
            &self.values[trim..self.values.len() - trim]
        } else {
            &self.values[..]
        };

        // Calculate the mean
        let mean = values.iter().fold(Decimal::ZERO, |sum, value| sum + *value) / Decimal::new(values.len() as i64, 0);
        Ok(Some(mean.normalize().to_string()))
    }
}

#[register]
impl AggregateUdf for Trimmean {
    fn clear(&mut self, _cfg: &UdfCfg<Process>, _error: Option<NonZeroU8>) -> Result<(), NonZeroU8> {
        self.values.clear();
        Ok(())
    }

    fn add(
        &mut self,
        _cfg: &UdfCfg<Process>,
        args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<(), NonZeroU8> {
        // Convert the 1st argument into Decimal
        let value = match args.get(0).unwrap().value() {
            SqlResult::Int(Some(value)) => Some(Decimal::new(value, 0)),
            SqlResult::Real(Some(value)) => Decimal::from_f64(value),
            SqlResult::Decimal(Some(value)) => Decimal::from_str_exact(value).ok(),
            _ => None,
        };

        // Skip values doesn't convert into Decimal
        let value = match value {
            Some(value) => value,
            None => return Err(NonZeroU8::new(1).unwrap()),
        };

        self.values.push(value);

        Ok(())
    }
}
