# Trimmean plugin for MariaDB/MySQL

![GitHub](https://img.shields.io/github/license/AkiraMiyakoda/mariadb-trimmean-rs)
[![Rust](https://github.com/AkiraMiyakoda/mariadb-trimmean-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/AkiraMiyakoda/mariadb-trimmean-rs/actions/workflows/rust.yml)

This is a MariaDB/MySQL plugin offers an aggregate function inspired by TRIMMEAN() of Microsoft Excel.

## Description

TRIMMEAN is an aggregate function returns the mean of the interior of a data set. TRIMMEAN calculates the mean taken by excluding a percentge of data points from the top and bottom tails of a data set.

## Syntax

### TRIMMEAN(expr, exclude_proportion)

- **expr** Required. The values to trim and average. NULL is excluded from the data set before calculating the number of excluded data points.

- **exclude_proportion** Required. The fractional number of data points to exclude from the calculation. For example, if `exclude_proportion` = 0.2, 4 points are trimmed from a data set of 20 points (20 x 0.2): 2 from the top and 2 from the bottom of the set.

## Remarks

TRIMMEAN rounds the number of excluded data points down to the nearest multiple of 2. If `exclude_proportion` = 0.1, 10 percent of 30 data points equals 3 points. For symmetry, TRIMMEAN excludes a single value from the top and bottom of the data set.

## Dependencies

- cargo

## Installation

### From source

You need to locate your MariaDB's plugin directory first, by executing `SELECT @@plugin_dir;` in your MariaDB client.

Once you have your plugin directory, follow these steps.

```console
cargo build --release
sudo cp ./target/release/libtrimmean_plugin.so <Your plugin directory>
```

Don't forget to run this statement to define the function in your MariaDB client.

```sql
CREATE AGGREGATE FUNCTION trimmean RETURNS DECIMAL SONAME 'libtrimmean_plugin.so';
```

## Example

```sql
CREATE TABLE test (x REAL);
INSERT INTO test (x) VALUES (1), (2), (4), (8), (16), (32), (64), (128), (256), (512);

SELECT TRIMMEAN(x, 0.1) FROM test; <- 102.3  = 1023 / 10
SELECT TRIMMEAN(x, 0.2) FROM test; <-  63.75 =  510 /  8
SELECT TRIMMEAN(x, 0.3) FROM test; <-  63.75 =  510 /  8
SELECT TRIMMEAN(x, 0.4) FROM test; <-  42    =  252 /  6
SELECT TRIMMEAN(x, 0.5) FROM test; <-  42    =  252 /  6
SELECT TRIMMEAN(x, 0.6) FROM test; <-  30    =  120 /  4
SELECT TRIMMEAN(x, 0.7) FROM test; <-  30    =  120 /  4
SELECT TRIMMEAN(x, 0.8) FROM test; <-  24    =   48 /  2
SELECT TRIMMEAN(x, 0.9) FROM test; <-  24    =   48 /  2
```
