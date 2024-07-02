# RustTI

A configurable technical indicators Rust library for all of your technical analysis needs.

Everything is configurable in the RustTI functions.

Many of the functions accept parameters that allow the caller to move the technical
indicators away from their default behaviour. For example, if a TI uses the mean to calculate
the indicator, it can be told to use the median, or mode instead.

For this reason, RustTI is a more advanced Technical Inidcators package, and users should
have some knowledge of the indicators they plan on using.

* [Install](#install)
* [Documentation](#documentation)
* [Examples](#examples)
* [Available indicators](#available-indicators)
* [Release notes](#release-notes)

## Install

Run the following Cargo command in your project directory:

```shell
cargo add rust_ti
```

Or add the following line to your Cargo.toml:

```
rust_ti = "1.1.0" 
```


## Documentation

Documentation can be found here: [rust_ti](https://docs.rs/rust_ti/latest/rust_ti/)


## Examples

### Simple Example

Single example, where the moving average needs to be calculated for the entire vector

```rust
use rust_ti;

let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];

let ma = rust_ti::moving_average::single::moving_average(
    &prices,
    &rust_ti::MovingAverageType::Simple
);
assert_eq!(100.352, ma);

let sma = rust_ti::moving_average::single::moving_average(
    &prices,
    &rust_ti::MovingAverageType::Smoothed
);
assert_eq!(100.34228938600666, sma);

let ema = rust_ti::moving_average::single::moving_average(
    &prices,
    &rust_ti::MovingAverageType::Exponential
);
assert_eq!(100.32810426540287, ema);

// The values used in the example for the personalised moving average are random.
// If using the PMA, it is recommended to look into how the moving averages are calculated before using values.
let pma = rust_ti::moving_average::single::moving_average(
    &prices,
    &rust_ti::MovingAverageType::Personalised(&5.0, &3.0)
);
assert_eq!(100.27405995388162, personalised_ma);
```

Bulk example, where the moving average is calculated for a period.

Behind the scenes, the function will be calculating the moving average for a period of 3. The slices 
`[100.2, 100.46, 100.53]`, `[100.46, 100.53, 100.38]`, and `[100.53, 100.38, 100.19]` will be derived from
`prices`. The function will return the moving averages for the 3 periods as a vector.

```rust
let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
let period: usize = 3;

let ma = rust_ti::moving_average::bulk::moving_average(
    &prices,
    &rust_ti::MovingAverageType::Simple,
    &period
);
assert_eq!(
    vec![100.39666666666666, 100.456666666666666, 100.36666666666667],
    ma
);

let sma = rust_ti::moving_average::bulk::moving_average(
    &prices,
    &rust_ti::MovingAverageType::Smoothed,
    &period
);
assert_eq!(
    vec![100.43842105263158, 100.4442105263158, 100.32157894736842],
    sma
);

let ema = rust_ti::moving_average::bulk::moving_average(
    &prices,
    &rust_ti::MovingAverageType::Exponential,
    &period
);
assert_eq!(
    vec![100.46285714285715, 100.4342857142857, 100.29285714285713],
    pma
);

let pma = rust_ti::moving_average::bulk::moving_average(
    &prices,
    &rust_ti::MovingAverageType::Personalised(&5.0, &3.0),
    &period
);
assert_eq!(
    vec![100.5125581395349, 100.40279069767443, 100.22441860465118],
    pma
);
```

### S&P 500 Example

An example using the Rust TI for the S&P 500 can be found in [GitHub](https://github.com/0100101001010000/rustTI/blob/main/examples/main.rs)

and under `examples/main.rs`

The code in `examples/main.rs` can be run by cloning the repo, and running:
```shell
cargo build
cargo run --example sp500
```


## Available indicators:

All indicators are grouped and split into modules based on their analysis area.

The modules are split into two sub modules: `bulk` and `single`. 

`Bulk` indicators calculate the indicator for a given period and return a vector of the indicator.

`Single` indicators calculate the indicator for the entire vector and return a single value. 

### Standard Indicators

Standard indicators are indicators with the configurations hardcoded to meet industry defaults.

* Simple Moving Average
* Smoothed Moving Average
* Exponential Moving Average
* Bollinger Bands
* MACD
* RSI

### Basic Indicators

Basic indicators are very simple indicators primarily used by other indicators. 
         
* Absolute Deviation
* Log
* Log Difference
* Mean 
* Median 
* Mode 
* Standard Deviation 
* Variance
* Max
* Min
 
### Candle Indicators

Candle indicators are indicators to be used with candle charts

* Ichimoku Cloud 
* McGinley Dynamic Bands 
* McGinley Dynamic Envelopes 
* Moving Constant Bands, generic version of Bollinger bansa 
* Moving Constant envelopes, generic version of moving average envelopes 
* Donchian Channels
* Keltner Channel

### Chart Trends

Chart trends are indicators to be used with charts that show trend direction

* Break down trends 
* Overall trend 
* Peak trend 
* Peaks 
* Valley trend 
* Valleys 

### Correlation indicators

Correlation calculate the correlation between two assests

* Correlate asset prices 

### Momentum Indicators

Momentum indicators calculate the momentum of price movement

* Chaikin Oscillator
* Commodity Channel Index 
* MACD Line 
* McGinley Dynamic Chaikin Oscillator 
* McGinley Dynamic Commodity Channel Index 
* McGinley Dynamic MACD Line 
* McGinley Dynamic RSI  
* Money Flow Index
* On Balance Volume 
* Rate of Change 
* Relative Strength Index 
* Signal Line 
* Slow Stochastic 
* Slowest Stochastic
* Stochastic Oscillator 
* Williams %r

### Moving Averages

* McGinley Dynamic
* Moving Average

### Other Indicators

Indicators that don't belong anywhere else

* Return on Investment
* True Range
* Average True Range

### Strength Indicators

Strength indicators calculate the strength of price movement

* Accumulation Distribution

### Trend Indicators

Trend indicators show the trend of price movement

* Aroon Down
* Aroon Indicator 
* Aroon Oscillator
* Aroon Up
* Parabolic Time Price System 
* Long Parabolic Time Price System
* Short Parabolic Time Price System 
* Direcational Movement
* Volume-Price trend

### Volatility Indicators

Volatility indicators show how volatile an asset is

* Ulcer Index
* Volatility System


## Release notes

What's new in v1.1.0?

* Added indicators from Welles "New concepts in technical trading systems" Directional Movement chapter

[Full changelog](https://github.com/0100101001010000/RustTI/releases/tag/v1.1.0)
