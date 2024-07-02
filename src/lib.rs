//! # RustTI
//!
//! A Technical Indicators library for Rust
//!
//! What differentiates RustTI from other Technical Indicator packages is how configurable the
//! functions are.
//!
//! Many models were created decades ago when the work weeks were different (such as RSI, SO,
//! Ichimoku cloud...). For example, if one studies
//!  stocks the work week is 5 days, but if one studies cryptocurrencies the work week is 7 days.
//!
//! Everything is configurable in the RustTI functions, from period to moving average models.
//!
//! A lot of online articles recommend sticking with the TI defaults as these are used by
//! traders. I would recommend the opposite. While it is true that most day
//! traders tend to stick to the defaults, large financial institutions such as
//! investment banks and hedge funds have their own quantitative teams who build them custom
//! models. This is what this package allows you to do.
//!
//! Many of the functions accept parameters that allow the caller to move the technial
//! indicators away from its default behaviour. For example, if a TI uses the mean to calculate
//! part of the indicator, it can be told to use the median, or mode instead.
//!
//! For this reason, RustTI is a more advanced Technical Inidcators package, and the users should
//! have some basic knowledge of the indicators they plan on using.
//!
//! RustTI is split into different modules, organised by common TI areas. Each module is then split
//! into a `single` and a `bulk` submodule.
//!
//! The `single` submodule is used to calculate the indicator once, using the entire price slice
//! that is passed in.
//!
//! The `bulk` submodule is used to iterate over a slice of prices to calculate the indicator for a period.
//!
//! ## Available indicators:
//!
//! ### Standard Indicators
//!
//! [`standard_indicators`] are configurable indicators that have had their configurations hardcoded to
//! follow industry standards.
//!
//! These are essentially wrappers for the other functions.
//!
//! #### Bulk
//!
//! * [`simple_moving_average`](standard_indicators::bulk::simple_moving_average)
//! * [`smoothed_moving_average`](standard_indicators::bulk::smoothed_moving_average)
//! * [`exponential_moving_average`](standard_indicators::bulk::exponential_moving_average)
//! * [`bollinger_bands`](standard_indicators::bulk::bollinger_bands)
//! * [`macd`](standard_indicators::bulk::macd)
//! * [`rsi`](standard_indicators::bulk::rsi)
//!
//! #### Single
//!
//! * [`simple_moving_average`](standard_indicators::single::simple_moving_average)
//! * [`smoothed_moving_average`](standard_indicators::single::smoothed_moving_average)
//! * [`exponential_moving_average`](standard_indicators::single::exponential_moving_average)
//! * [`bollinger_bands`](standard_indicators::single::bollinger_bands)
//! * [`macd`](standard_indicators::single::macd)
//! * [`rsi`](standard_indicators::single::rsi)
//!
//! ### Basic Indicators
//! [`basic_indicators`] are basic functions that are often the foundation of more complex
//! indicators.
//!
//! #### Bulk
//!          
//! * [`absolute_deviation`](basic_indicators::bulk::absolute_deviation) - Calculates the absolute deviation
//! * [`log`](basic_indicators::bulk::log) - Calculates the natural logrithm of slice of prices
//! * [`log_difference`](basic_indicators::bulk::log_difference) - Calculates the difference
//! between the natural logarithm at t and t-1
//! * [`mean`](basic_indicators::bulk::mean) - Calculates the mean (average) of a slice of prices
//! * [`median`](basic_indicators::bulk::median) - Calculates the median (middle value) of a slice of prices
//! * [`mode`](basic_indicators::bulk::mode) - Calculates the mode (most common price) of a slice of prices
//! * [`standard_deviation`](basic_indicators::bulk::standard_deviation) - Calculates the standard
//! deviation of a slice of prices
//! * [`variance`](basic_indicators::bulk::variance) - Calculates the variance of slice of prices
//!
//! #### Single
//!
//! * [`absolute_deviation`](basic_indicators::single::absolute_deviation) - Calculates the absolute deviation
//! * [`log_difference`](basic_indicators::single::log_difference) - Calculates the difference
//! between the natural logarithm at t and t-1
//! * [`max`](basic_indicators::single::max) - Calculates the maximum of a slice of prices
//! * [`mean`](basic_indicators::single::mean) - Calculates the mean (average) of a slice of prices
//! * [`median`](basic_indicators::single::median) - Calculates the median (middle value) of a slice of prices
//! * [`min`](basic_indicators::single::min) - Calculates the minimum of a slice of prices
//! * [`mode`](basic_indicators::single::mode) - Calculates the mode (most common price) of a slice of prices
//! * [`standard_deviation`](basic_indicators::single::standard_deviation) - Calculates the
//!  standard deviation of a slice of prices
//! * [`variance`](basic_indicators::single::variance) - Calculates the variance of slice of prices
//!
//! ### Candle Indicators
//! [`candle_indicators`] are indicators that are used with candle charts.
//!
//! #### Bulk
//!
//! * [`ichimoku_cloud`](candle_indicators::bulk::ichimoku_cloud) - Calculates the Ichimoku Cloud
//! * [`mcginley_dynamic_bands`](candle_indicators::bulk::mcginley_dynamic_bands) - The McGinley Dynamic
//! version of the [`moving_constant_bands`](candle_indicators::bulk::moving_constant_bands)
//! * [`mcginley_dynamic_envelopes`](candle_indicators::bulk::mcginley_dynamic_envelopes) - The McGinley
//!  Dynamic version of the
//! * [`moving_constant_envelopes`](candle_indicators::bulk::moving_constant_envelopes)
//! * [`moving_constant_bands`](candle_indicators::bulk::moving_constant_bands) - Calculates the moving
//!  constant bands
//! * [`moving_constant_envelopes`](candle_indicators::bulk::moving_constant_envelopes) - Calculates the moving
//!  constant envelopes
//! * [`donchian_channels`](candle_indicators::bulk::donchian_channels)
//! * [`keltner_channel`](candle_indicators::bulk::keltner_channel)
//!
//! #### Single
//!
//! * [`ichimoku_cloud`](candle_indicators::single::ichimoku_cloud) - Calculates the Ichimoku Cloud
//! * [`mcginley_dynamic_bands`](candle_indicators::single::mcginley_dynamic_bands) - The McGinley Dynamic
//!  version of the [`moving_constant_bands`](candle_indicators::single::moving_constant_bands)
//! * [`mcginley_dynamic_envelopes`](candle_indicators::single::mcginley_dynamic_envelopes) - The McGinley
//!  Dynamic version of the
//! * [`moving_constant_envelopes`](candle_indicators::single::moving_constant_envelopes)
//! * [`moving_constant_bands`](candle_indicators::single::moving_constant_bands) - Calculates
//! the moving constant bands
//! * [`moving_constant_envelopes`](candle_indicators::single::moving_constant_envelopes) - Calculates the moving
//! * [`donchian_channels`](candle_indicators::single::donchian_channels)
//! * [`keltner_channel`](candle_indicators::single::keltner_channel)
//!
//! ### Chart Trends
//! [`chart_trends`] shows trends on charts. Unlike the other modules it has no bulk or single.
//!
//! * [`break_down_trends`](chart_trends::break_down_trends) - Breaks down the chart into different
//! price trends
//! * [`overall_trend`](chart_trends::overall_trend) - Calculates the overall trend for all price
//! points
//! * [`peak_trend`](chart_trends::peak_trend) - Calculates the trend of the peaks
//! * [`peaks`](chart_trends::peaks) - Returns all the peaks
//! * [`valley_trend`](chart_trends::valley_trend) - Calculates trend of the valleys
//! * [`valleys`](chart_trends::valleys) - Returns all the valleys
//!
//! ### Correlation indicators
//!
//! [`correlation_indicators`] show how closely the prices of two different assets move together.
//!
//! #### Bulk
//!
//! * [`correlate_asset_prices`](correlation_indicators::bulk::correlate_asset_prices) - Calculates
//! the correlation between two assets
//!
//! #### Single
//!
//! * [`correlate_asset_prices`](correlation_indicators::single::correlate_asset_prices) - Calculates
//! the correlation between two assets
//!
//! ### Momentum Indicators
//!
//! [`momentum_indicators`] show how much the price is rising or falling
//!
//! #### Bulk
//!
//! * [`chaikin_oscillator`](momentum_indicators::bulk::chaikin_oscillator) - Calculates the Chaikin Oscillator
//! * [`commodity_channel_index`](momentum_indicators::bulk::commodity_channel_index) - Calculate
//! the Commodity Channel Index
//! * [`macd_line`](momentum_indicators::bulk::macd_line) - Calculates the Moving Average Convergence
//! Divergence line
//! [`mcginley_dynamic_chaikin_oscillator`](momentum_indicators::bulk::mcginley_dynamic_chaikin_oscillator) -
//! Calculate the McGinley dynanic version of the Chaikin Oscillator
//! * [`mcginley_dynamic_commodity_channel_index`](momentum_indicators::bulk::mcginley_dynamic_commodity_channel_index) -
//!  Calculates the McGinley dynamic version of the Commodity Channel Index
//! * [`mcginley_dynamic_macd_line`](momentum_indicators::bulk::mcginley_dynamic_macd_line) -
//!  Calculates the McGinley dynamic version of the Moving Average Convergence Divergence line
//! * [`mcginley_dynamic_rsi`](momentum_indicators::bulk::mcginley_dynamic_rsi) - Calculates
//! the McGinley dynamic version of the Relative Strength Index
//! * [`money_flow_index`](momentum_indicators::bulk::money_flow_index) - Calculates the Money Flow Index
//! * [`on_balance_volume`](momentum_indicators::bulk::on_balance_volume) - Calculates the On-balance Volume
//! * [`rate_of_change`](momentum_indicators::bulk::rate_of_change) - Calculates the Rate of Change
//! * [`relative_strength_index`](momentum_indicators::bulk::relative_strength_index) - Calculates
//! the Relative Strength Index
//! * [`signal_line`](momentum_indicators::bulk::signal_line) - Calculates the Signal line to be
//!  used with the MACD line
//! * [`slow_stochastic`](momentum_indicators::bulk::slow_stochastic) - Calculates the slow stochastic
//! to be used with the stochastic oscillator
//! * [`slowest_stochastic`](momentum_indicators::bulk::slowest_stochastic) - Calculates the
//!  slowest stochastic to be used with the stochastic oscillator
//! * [`stochastic_oscillator`](momentum_indicators::bulk::stochastic_oscillator) - Calculates the
//! Stochastic Oscillator
//! * [`williams_percent_r`](momentum_indicators::bulk::williams_percent_r) - Calcualtes the Williams %R
//!
//! #### Single
//!
//! * [`chaikin_oscillator`](momentum_indicators::single::chaikin_oscillator) - Calculates the Chaikin Oscillator
//! * [`commodity_channel_index`](momentum_indicators::single::commodity_channel_index) - Calculate the
//!  Commodity Channel Index
//! * [`macd_line`](momentum_indicators::single::macd_line) - Calculates the Moving Average
//!  Convergence Divergence line
//! * [`mcginley_dynamic_chaikin_oscillator`](momentum_indicators::single::mcginley_dynamic_chaikin_oscillator) -
//!  Calculate the McGinley dynanic version of the Chaikin Oscillator
//! * [`mcginley_dynamic_commodity_channel_index`](momentum_indicators::single::mcginley_dynamic_commodity_channel_index) -
//!  Calculates the McGinley dynamic version of the Commodity Channel Index
//! * [`mcginley_dynamic_macd_line`](momentum_indicators::single::mcginley_dynamic_macd_line) -
//!  Calculates the McGinley dynamic version of the Moving Average Convergence Divergence line
//! * [`mcginley_dynamic_rsi`](momentum_indicators::single::mcginley_dynamic_rsi) - Calculates
//!  the McGinley dynamic version of the Relative Strength Index
//! * [`money_flow_index`](momentum_indicators::single::money_flow_index) - Calculates the Money Flow Index
//! * [`on_balance_volume`](momentum_indicators::single::on_balance_volume) - Calculates the On-balance Volume
//! * [`rate_of_change`](momentum_indicators::single::rate_of_change) - Calculates the Rate of Change
//! * [`relative_strength_index`](momentum_indicators::single::relative_strength_index) - Calculates
//!  the Relative Strength Index
//! * [`signal_line`](momentum_indicators::single::signal_line) - Calculates the Signal line
//!  to be used with the MACD line
//! * [`slow_stochastic`](momentum_indicators::single::slow_stochastic) - Calculates the
//!  slow stochastic to be used with the stochastic oscillator
//! * [`slowest_stochastic`](momentum_indicators::single::slowest_stochastic) - Calculates the
//!  slowest stochastic to be used with the stochastic oscillator
//! * [`stochastic_oscillator`](momentum_indicators::single::stochastic_oscillator) - Calculates
//! the Stochastic Oscillator
//! * [`williams_percent_r`](momentum_indicators::single::williams_percent_r) - Calcualtes the Williams %R
//!
//! ### Moving Averages
//!
//! The [`moving_average`] module has functions used to calculate the moving average
//!
//! #### Bulk
//!
//! * [`mcginley_dynamic`](moving_average::bulk::mcginley_dynamic) - Calculates the McGinley Dynamic
//! * [`moving_average`](moving_average::bulk::moving_average) - Calculates different types Moving Averages
//!
//! #### Single
//!
//! * [`mcginley_dynamic`](moving_average::single::mcginley_dynamic) - Calculates the McGinley Dynamic
//! * [`moving_average`](moving_average::single::moving_average) - Calculates different types Moving Averages
//!
//! ### Other Indicators
//!
//! [`other_indicators`] don't really fit in anywhere else
//!
//! #### Bulk
//!
//! * [`return_on_investment`](other_indicators::bulk::return_on_investment) - Calculates the return
//! on investment
//! * [`true_range`](other_indicators::bulk::true_range)
//! * [`average_true_range`](other_indicators::bulk::average_true_range)
//!
//! #### Single
//!
//! * [`return_on_investment`](other_indicators::single::return_on_investment) - Calculates the return
//! on investment
//! * [`true_range`](other_indicators::bulk::true_range)
//! * [`average_true_range`](other_indicators::bulk::average_true_range)
//!
//! ### Strength Indicators
//!
//! [`strength_indicators`] show the strength of a trend
//!
//! #### Bulk
//!
//! * [`accumulation_distribution`](strength_indicators::bulk::accumulation_distribution) - Calculates
//! the Accumulation Distribution
//!
//! #### Single
//!
//! * [`accumulation_distribution`](strength_indicators::single::accumulation_distribution) - Calculates
//! the Accumulation Distribution
//!
//! ### Trend Indicators
//!
//! [`trend_indicators`] show the trend direction of an asset
//!
//! #### Bulk
//!
//! * [`aroon_down`](trend_indicators::bulk::aroon_down) - Calculates the Aroon down
//! * [`aroon_indicator`](trend_indicators::bulk::aroon_indicator) - Calculates the Aroon indicator
//! * [`aroon_oscillator`](trend_indicators::bulk::aroon_oscillator) - Calculates the Aroon Oscillator
//! * [`aroon_up`](trend_indicators::bulk::aroon_up) - Calculates the Aroon up
//! * [`parabolic_time_price_system`](trend_indicators::bulk::parabolic_time_price_system) - Calculates
//! the parabolic time price system
//! * [`volume_price_trend`](trend_indicators::bulk:volume_price_trend)
//!
//! #### Single
//!
//! * [`aroon_down`](trend_indicators::single::aroon_down) - Calculates the Aroon down
//! * [`aroon_indicator`](trend_indicators::single::aroon_indicator) - Calculates the Aroon indicator
//! * [`aroon_oscillator`](trend_indicators::single::aroon_oscillator) - Calculates the Aroon Oscillator
//! * [`aroon_up`](trend_indicators::single::aroon_up) - Calculates the Aroon up
//! * [`long_parabolic_time_price_system`](trend_indicators::single::long_parabolic_time_price_system) -
//!  Calculates the parabolic time price system for long positions
//! * [`short_parabolic_time_price_system`](trend_indicators::single::short_parabolic_time_price_system) -
//!  Calculates the parabolic time price system for short positions
//! * [`volume_price_trend`](trend_indicators::single:volume_price_trend)
//!
//! ### Volatility Indicators
//!
//! [`volatility_indicators`] show how volatile an asset are.
//!
//! #### Bulk
//!
//! * [`ulcer_index`](volatility_indicators::bulk::ulcer_index) - Calculates the Ulcer Index
//! * [`voltility_system`](volatility_indicators::bulk::volatility_system) - Calculates Welles
//! volatility system
//!
//! #### Single
//!
//! * [`ulcer_index`](volatility_indicators::single::ulcer_index) - Calculates the Ulcer Index
//! * [`voltility_system`](volatility_indicators::single::volatility_system) - Calculates Welles
//! volatility system

pub mod basic_indicators;
pub mod candle_indicators;
pub mod chart_trends;
pub mod correlation_indicators;
pub mod momentum_indicators;
pub mod moving_average;
pub mod other_indicators;
pub mod standard_indicators;
pub mod strength_indicators;
pub mod trend_indicators;
pub mod volatility_indicators;

/// The `CentralPoint enum` is used to determine the central point
#[derive(Debug)]
pub enum CentralPoint {
    Mean,
    Median,
    Mode,
}

/// The `MovingAverageType` is used when calculating the `moving_average`.
/// The simple caclculates the moving average without interferring, however the smoothed and
/// exponential types assign weights to the more recent prices.
///
/// Personalised allows the caller the influence the weighting calculation, some research should be
/// done in how the alpha is calculated in different types before using this. The first float is
/// the alpha nominator, and the second float is the alpha denominator. The smoothed type
/// uses an alpha nominator of 1, and denominator of 0. The exponential type uses an alpha nominator
/// of 2, and denominator of 1. Probably shouldn't be used...
#[derive(Debug)]
pub enum MovingAverageType<'a> {
    Simple,
    Smoothed,
    Exponential,
    Personalised(&'a f64, &'a f64),
}

/// The `ConstantModelType` enum is used by a number of functions to determine the centerpoint around
/// which to do its calculations.
///
/// Most of the time it uses a flavor of the moving average, but more are provided here to give the
/// caller the opportunity to diversify the functions a bit.
///
/// See note in `MovingAverageType` about using the `Personalised` variant.
#[derive(Debug)]
pub enum ConstantModelType<'a> {
    SimpleMovingAverage,
    SmoothedMovingAverage,
    ExponentialMovingAverage,
    PersonalisedMovingAverage(&'a f64, &'a f64),
    SimpleMovingMedian,
    SimpleMovingMode,
}

/// The `DeviationModel` enum is used by a number of functions to determine the deviation from a central
/// point.
#[derive(Debug)]
pub enum DeviationModel {
    StandardDeviation,
    MeanAbsoluteDeviation,
    MedianAbsoluteDeviation,
    ModeAbsoluteDeviation,
    UlcerIndex,
}

/// The `Position` enum is used for indicators that need to know the position of the trade (short,
/// long) to perform their calculations.
#[derive(Debug)]
pub enum Position {
    Short,
    Long,
}
