// Copyright © 2023 Akira Miyakoda
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use udf::prelude::*;

#[derive(Debug)]
pub(crate) struct Trimmean {
    values: Vec<f64>,
    proportion: f64,
}

#[register]
impl BasicUdf for Trimmean {
    type Returns<'a> = Option<f64>;

    fn init(cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
        // Check the number and types of args
        if args.len() != 2 {
            return Err(format!("Expected 2 args, but got {}", args.len()));
        }

        // Check if the args are in correct types
        let arg0 = args.get(0).unwrap().value();
        if arg0.is_string() {
            return Err(String::from("1st arg must be real, decimal or int"));
        }

        let arg1 = args.get(1).unwrap().value();
        if arg1.is_string() || arg1.is_int() {
            return Err(String::from("2nd arg must be real or decimal"));
        }

        // Check if the 2nd arg is valid
        let proportion = match arg1 {
            SqlResult::Real(Some(v)) => Some(v),
            SqlResult::Decimal(Some(v)) => v.parse::<f64>().ok(),
            _ => None,
        };
        let proportion = match proportion {
            Some(prop) => prop,
            None => return Err(String::from("Failed to convert 2nd arg into real")),
        };
        if proportion < 0.0 || 1.0 <= proportion {
            return Err(String::from("2nd arg out of range (0, 1)"));
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
        let trim = (self.values.len() as f64 * self.proportion) as usize / 2;
        let values = if trim > 0 {
            // We can assume all the elements are finite
            self.values
                .sort_unstable_by(|a, b| a.partial_cmp(&b).unwrap());
            let pos = (trim, self.values.len() - trim);
            &self.values[pos.0..pos.1]
        } else {
            &self.values[..]
        };

        let mean = values.iter().fold(0.0, |sum, v| sum + v) / values.len() as f64;
        Ok(Some(mean))
    }
}

#[register]
impl AggregateUdf for Trimmean {
    fn clear(
        &mut self,
        _cfg: &UdfCfg<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<(), NonZeroU8> {
        self.values.clear();
        Ok(())
    }

    fn add(
        &mut self,
        _cfg: &UdfCfg<Process>,
        args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<(), NonZeroU8> {
        // Convert the 1st argument into f64
        let value = match args.get(0).unwrap().value() {
            SqlResult::Int(Some(v)) => Some(v as f64),
            SqlResult::Real(Some(v)) => Some(v),
            SqlResult::Decimal(Some(v)) => v.parse::<f64>().ok(),
            _ => None,
        };

        // Skip values doesn't convert into f64
        if value.is_none() {
            return Err(NonZeroU8::new(1).unwrap());
        }

        self.values.push(value.unwrap());

        Ok(())
    }
}
