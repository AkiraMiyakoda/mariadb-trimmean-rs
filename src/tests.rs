// Copyright © 2024 Akira Miyakoda
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use udf::{mock::*, AggregateUdf, BasicUdf};

use crate::trimmean::Trimmean;

#[test]
fn real() {
    let mut cfg = MockUdfCfg::new();

    let mut rows = [
        mock_args![(Real 1.001, "", true), (Real 0.3, "", false)],
        mock_args![(Real 2.002, "", true), (Real None, "", false)],
        mock_args![(Real 4.004, "", true), (Real None, "", false)],
        mock_args![(Real 8.008, "", true), (Real None, "", false)],
        mock_args![(Real 16.016, "", true), (Real None, "", false)],
        mock_args![(Real 32.032, "", true), (Real None, "", false)],
        mock_args![(Real 64.064, "", true), (Real None, "", false)],
        mock_args![(Real 128.128, "", true), (Real None, "", false)],
        mock_args![(Real 256.256, "", true), (Real None, "", false)],
        mock_args![(Real 512.512, "", true), (Real None, "", false)],
    ];

    let mut func = Trimmean::init(cfg.as_init(), rows[0].as_init()).unwrap();
    for i in 0..rows.len() {
        func.add(cfg.as_process(), rows[i].as_process(), None)
            .unwrap();
    }
    let mean = func.process(cfg.as_process(), rows[9].as_process(), None);

    assert_eq!(mean, Ok(Some("63.81375".into())));
}

#[test]
fn int() {
    let mut cfg = MockUdfCfg::new();

    let mut rows = [
        mock_args![(Int 1, "", true), (Real 0.5, "", false)],
        mock_args![(Int 2, "", true), (Real None, "", false)],
        mock_args![(Int 4, "", true), (Real None, "", false)],
        mock_args![(Int 8, "", true), (Real None, "", false)],
        mock_args![(Int 16, "", true), (Real None, "", false)],
        mock_args![(Int 32, "", true), (Real None, "", false)],
        mock_args![(Int 64, "", true), (Real None, "", false)],
        mock_args![(Int 128, "", true), (Real None, "", false)],
        mock_args![(Int 256, "", true), (Real None, "", false)],
        mock_args![(Int 512, "", true), (Real None, "", false)],
    ];

    let mut func = Trimmean::init(cfg.as_init(), rows[0].as_init()).unwrap();
    for i in 0..rows.len() {
        func.add(cfg.as_process(), rows[i].as_process(), None)
            .unwrap();
    }
    let mean = func.process(cfg.as_process(), rows[9].as_process(), None);

    assert_eq!(mean, Ok(Some("42".into())));
}

#[test]
fn decimal() {
    let mut cfg = MockUdfCfg::new();

    let mut rows = [
        mock_args![(Decimal "1.001", "", true), (Decimal "0.7", "", false)],
        mock_args![(Decimal "2.002", "", true), (Decimal None, "", false)],
        mock_args![(Decimal "4.004", "", true), (Decimal None, "", false)],
        mock_args![(Decimal "8.008", "", true), (Decimal None, "", false)],
        mock_args![(Decimal "16.016", "", true), (Decimal None, "", false)],
        mock_args![(Decimal "32.032", "", true), (Decimal None, "", false)],
        mock_args![(Decimal "64.064", "", true), (Decimal None, "", false)],
        mock_args![(Decimal "128.128", "", true), (Decimal None, "", false)],
        mock_args![(Decimal "256.256", "", true), (Decimal None, "", false)],
        mock_args![(Decimal "512.512", "", true), (Decimal None, "", false)],
    ];

    let mut func = Trimmean::init(cfg.as_init(), rows[0].as_init()).unwrap();
    for i in 0..rows.len() {
        func.add(cfg.as_process(), rows[i].as_process(), None)
            .unwrap();
    }
    let mean = func.process(cfg.as_process(), rows[9].as_process(), None);

    assert_eq!(mean, Ok(Some("30.03".into())));
}

#[test]
fn invalid_args() {
    let mut cfg = MockUdfCfg::new();

    let mut row = mock_args![(Real 1.0, "", true)];
    let func = Trimmean::init(cfg.as_init(), row.as_init());
    assert_eq!(func.unwrap_err(), "Expected 2 args, but got 1");

    let mut row = mock_args![
        (Real 1.0, "", true), (Real 1.0, "", true), (Real 1.0, "", true)
    ];
    let func = Trimmean::init(cfg.as_init(), row.as_init());
    assert_eq!(func.unwrap_err(), "Expected 2 args, but got 3");

    let mut row = mock_args![(String "1.00", "", true), (Real 0.3, "", false)];
    let func = Trimmean::init(cfg.as_init(), row.as_init());
    assert_eq!(func.unwrap_err(), "1st arg must be real, decimal or int");

    let mut row = mock_args![(Real 1.0, "", true), (Int 0, "", false)];
    let func = Trimmean::init(cfg.as_init(), row.as_init());
    assert_eq!(func.unwrap_err(), "2nd arg must be real or decimal");

    let mut row = mock_args![(Real 1.0, "", true), (String "ABC", "", false)];
    let func = Trimmean::init(cfg.as_init(), row.as_init());
    assert_eq!(func.unwrap_err(), "2nd arg must be real or decimal");

    let mut row = mock_args![(Real 1.0, "", true), (Decimal "ABC", "", false)];
    let func = Trimmean::init(cfg.as_init(), row.as_init());
    assert_eq!(func.unwrap_err(), "Failed to convert 2nd arg into decimal");

    let mut row = mock_args![(Real 1.0, "", true), (Decimal "-0.01", "", false)];
    let func = Trimmean::init(cfg.as_init(), row.as_init());
    assert_eq!(func.unwrap_err(), "2nd arg out of range [0.0, 1.0)");

    let mut row = mock_args![(Real 1.0, "", true), (Decimal "1.0", "", false)];
    let func = Trimmean::init(cfg.as_init(), row.as_init());
    assert_eq!(func.unwrap_err(), "2nd arg out of range [0.0, 1.0)");
}
