//! # Candle indicators
//!
//! Candle indicators are indicators that are used with candle charts.
//!
//! ## Bulk
//!
//! * [`ichimoku_cloud`](bulk::ichimoku_cloud) - Calculates the Ichimoku Cloud
//! * [`mcginley_dynamic_bands`](bulk::mcginley_dynamic_bands) - The McGinley Dynamic
//! version of the [`moving_constant_bands`](bulk::moving_constant_bands)
//! * [`mcginley_dynamic_envelopes`](bulk::mcginley_dynamic_envelopes) - The McGinley Dynamic version of the
//! [`moving_constant_envelopes`](bulk::moving_constant_envelopes)
//! * [`moving_constant_bands`](bulk::moving_constant_bands) - Calculates the moving constant bands
//! * [`moving_constant_envelopes`](bulk::moving_constant_envelopes) - Calculates the moving
//! constant envelopes
//! * [`donchian_channels`](bulk::donchian_channels)
//! * [`keltner_channel`](bulk::keltner_channel)
//!
//! ## Single
//!
//! * [`ichimoku_cloud`](single::ichimoku_cloud) - Calculates the Ichimoku Cloud
//! * [`mcginley_dynamic_bands`](single::mcginley_dynamic_bands) - The McGinley Dynamic
//! version of the [`moving_constant_bands`](single::moving_constant_bands)
//! * [`mcginley_dynamic_envelopes`](single::mcginley_dynamic_envelopes) - The McGinley Dynamic version of the
//! [`moving_constant_envelopes`](single::moving_constant_envelopes)
//! * [`moving_constant_bands`](single::moving_constant_bands) - Calculates the moving constant bands
//! * [`moving_constant_envelopes`](single::moving_constant_envelopes) - Calculates the moving
//! constant envelopes
//! * [`donchian_channels`](single::donchian_channels)
//! * [`keltner_channel`](single::keltner_channel)

/// `single` module holds functions that return a singular values
pub mod single {
    use crate::basic_indicators::single::{
        absolute_deviation, max, median, min, mode, standard_deviation,
    };
    use crate::moving_average::single::{mcginley_dynamic, moving_average};
    use crate::volatility_indicators::single::ulcer_index;
    use crate::{ConstantModelType, DeviationModel, MovingAverageType};
    use crate::other_indicators::single::average_true_range;
    /// The `moving_constant_envelopes` function calculates upper and lower bands from the
    /// moving constant of the price. The function returns a tuple with the lower band,
    /// moving constant, upper band (in that order).
    ///
    /// The standard is to use the moving averages to create a moving average envelope but this
    /// function has been extended to allow the caller to use the other constant model types, hence
    /// the name.
    ///
    /// The caller also determines the difference, or the distance between the price and the bands.
    /// The `int` passed in is a percentage, so passing in 3 would mean a 3% difference from the
    /// moving average.
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `constant_model_type` - Variant of the [`ConstantModelType`] enum.
    /// * `difference` - The percent difference or distance that the bands will be from the moving
    /// constant
    ///
    /// # Panics
    ///
    /// `moving_constant_envelopes` will panic if passed in an empty `prices` slice
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0];
    /// let difference = 3.0;
    ///
    /// let ema_envelope = rust_ti::candle_indicators::single::moving_constant_envelopes(&prices,
    /// &rust_ti::ConstantModelType::ExponentialMovingAverage, &difference);
    /// assert_eq!((97.59303317535547, 100.61137440758296, 103.62971563981044), ema_envelope);
    ///
    /// let median_envelope = rust_ti::candle_indicators::single::moving_constant_envelopes(&prices,
    /// &rust_ti::ConstantModelType::SimpleMovingMedian, &difference);
    /// assert_eq!((97.97, 101.0, 104.03), median_envelope);
    /// ```
    pub fn moving_constant_envelopes(
        prices: &[f64],
        constant_model_type: &ConstantModelType,
        difference: &f64,
    ) -> (f64, f64, f64) {
        if prices.is_empty() {
            panic!("Prices cannot be empty")
        };

        let moving_constant = match constant_model_type {
            ConstantModelType::SimpleMovingAverage => {
                moving_average(&prices, &MovingAverageType::Simple)
            }
            ConstantModelType::SmoothedMovingAverage => {
                moving_average(&prices, &MovingAverageType::Smoothed)
            }
            ConstantModelType::ExponentialMovingAverage => {
                moving_average(&prices, &MovingAverageType::Exponential)
            }
            ConstantModelType::PersonalisedMovingAverage(alpha_nominator, alpha_denominator) => {
                moving_average(
                    &prices,
                    &MovingAverageType::Personalised(alpha_nominator, alpha_denominator),
                )
            }
            ConstantModelType::SimpleMovingMedian => median(&prices),
            ConstantModelType::SimpleMovingMode => mode(&prices),
            _ => panic!("Unsupported ConstantModelType"),
        };

        let upper_envelope = &moving_constant * (1.0 + (difference / 100.0));
        let lower_envelope = &moving_constant * (1.0 - (difference / 100.0));
        return (lower_envelope, moving_constant, upper_envelope);
    }

    /// The `mcginley_dynamic_envelopes` function calculates upper and lower bands from the
    /// Mcginley dynamic of the price. The function returns a tuple with the lower band,
    /// McGinley dynamic, upper band (in that order).
    ///
    /// This is a variation of the `moving_constant_envelopes` function.
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `difference` - The percent difference or distance that the bands will be from the moving
    /// constant
    /// * `previous_mcginley_dynamic` - Previous value for the McGinley dynamic. 0.0 if no
    /// previous.
    ///
    /// # Panics
    ///
    /// The `mcginley_dynamic_envelopes` will panic if `prices` is empty
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0];
    /// let difference = 3.0;
    ///
    /// let mcginley_envelope = rust_ti::candle_indicators::single::mcginley_dynamic_envelopes(&prices,
    ///  &difference, &0.0);
    /// assert_eq!((96.03, 99.0, 101.97), mcginley_envelope);
    ///
    /// let prices = vec![102.0, 103.0, 101.0, 99.0, 102.0];
    /// let mcginley_envelope = rust_ti::candle_indicators::single::mcginley_dynamic_envelopes(&prices,
    ///  &difference, &mcginley_envelope.1);
    /// assert_eq!((96.54649137791692, 99.53246533805869, 102.51843929820045), mcginley_envelope);
    /// ```
    pub fn mcginley_dynamic_envelopes(
        prices: &[f64],
        difference: &f64,
        previous_mcginley_dynamic: &f64,
    ) -> (f64, f64, f64) {
        if prices.is_empty() {
            panic!("Prices cannot be empty!");
        };

        let last_price = prices.last().unwrap();
        let mcginley_dynamic =
            mcginley_dynamic(&last_price, previous_mcginley_dynamic, &prices.len());
        let upper_envelope = &mcginley_dynamic * (1.0 + (difference / 100.0));
        let lower_envelope = &mcginley_dynamic * (1.0 - (difference / 100.0));
        return (lower_envelope, mcginley_dynamic, upper_envelope);
    }

    /// The `moving_constant_bands` are inspired from the Bollinger Bands but have been extended to
    /// allow the caller to determine the moving constant model, the deviation model, and the
    /// multiplier of the deviation model.
    ///
    /// Returns a tuple with the lower band, moving constant, the upper band
    ///
    /// The standard for Bollinger Bands is to use a moving average model, the standard deviation,
    /// and a standard deviation model of 2.
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `constant_model_type` - Variant of the [`ConstantModelType`] enum.
    /// * `deviation_model` - A variant of the [`DeviationModel`] enum.
    /// * `deviation_multiplier` - Multiplier for the deviation of prices.
    ///
    /// # Panics
    ///
    /// `moving_constant_bands` will panic if `prices` is empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0];
    /// let multiplier = 2.0;
    ///
    /// let bollinger_bands = rust_ti::candle_indicators::single::moving_constant_bands(&prices,
    /// &rust_ti::ConstantModelType::SimpleMovingAverage, &rust_ti::DeviationModel::StandardDeviation, &multiplier);
    /// assert_eq!((98.17157287525382, 101.0, 103.82842712474618), bollinger_bands);
    ///
    /// let ema_mad_bands = rust_ti::candle_indicators::single::moving_constant_bands(&prices, &rust_ti::ConstantModelType::ExponentialMovingAverage, &rust_ti::DeviationModel::MeanAbsoluteDeviation, &multiplier);
    /// assert_eq!((98.21137440758295, 100.61137440758296, 103.01137440758296), ema_mad_bands);
    /// ```
    pub fn moving_constant_bands(
        prices: &[f64],
        constant_model_type: &ConstantModelType,
        deviation_model: &DeviationModel,
        deviation_multiplier: &f64,
    ) -> (f64, f64, f64) {
        if prices.is_empty() {
            panic!("Prices cannot be empty")
        };

        let moving_constant = match constant_model_type {
            ConstantModelType::SimpleMovingAverage => {
                moving_average(&prices, &MovingAverageType::Simple)
            }
            ConstantModelType::SmoothedMovingAverage => {
                moving_average(&prices, &MovingAverageType::Smoothed)
            }
            ConstantModelType::ExponentialMovingAverage => {
                moving_average(&prices, &MovingAverageType::Exponential)
            }
            ConstantModelType::PersonalisedMovingAverage(alpha_nominator, alpha_denominator) => {
                moving_average(
                    &prices,
                    &MovingAverageType::Personalised(alpha_nominator, alpha_denominator),
                )
            }
            ConstantModelType::SimpleMovingMedian => median(&prices),
            ConstantModelType::SimpleMovingMode => mode(&prices),
            _ => panic!("Unsupported ConstantModelType"),
        };

        let deviation = match deviation_model {
            DeviationModel::StandardDeviation => standard_deviation(&prices),
            DeviationModel::MeanAbsoluteDeviation => {
                absolute_deviation(&prices, &crate::CentralPoint::Mean)
            }
            DeviationModel::MedianAbsoluteDeviation => {
                absolute_deviation(&prices, &crate::CentralPoint::Median)
            }
            DeviationModel::ModeAbsoluteDeviation => {
                absolute_deviation(&prices, &crate::CentralPoint::Mode)
            }
            DeviationModel::UlcerIndex => ulcer_index(&prices),
            _ => panic!("Unsupported DeviationModel"),
        };
        let deviation_multiplied = deviation * deviation_multiplier;
        let upper_band = &moving_constant + &deviation_multiplied;
        let lower_band = &moving_constant - &deviation_multiplied;
        return (lower_band, moving_constant, upper_band);
    }

    /// The `mcginley_dynamic_bands` are a variation of the `moving_constant_bands` but uses
    /// the McGinley Dynamic rather than a moving constant model.
    ///
    /// Returns a tuple with the lower band, McGinley dynamic, the upper band
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `deviation_model` - Variant of the [`DeviationModel`] enum.
    /// * `deviation_multiplier` - Multiplier for the deviation of prices.
    /// * `previous_mcginley_dynamic` - Previous value for the McGinley dynamic. 0.0 if no
    /// previous.
    ///
    /// # Panics
    ///
    /// `mcginley_dynamic_bands` panics if `prices` is empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0];
    /// let multiplier = 2.0;
    ///
    /// let mcginley_bands = rust_ti::candle_indicators::single::mcginley_dynamic_bands(&prices,
    /// &rust_ti::DeviationModel::StandardDeviation, &multiplier, &0.0);
    /// assert_eq!((96.17157287525382, 99.0, 101.82842712474618), mcginley_bands);
    ///
    /// let prices = vec![102.0, 103.0, 101.0, 99.0, 102.0];
    /// let mcginley_bands = rust_ti::candle_indicators::single::mcginley_dynamic_bands(&prices,
    /// &rust_ti::DeviationModel::StandardDeviation, &multiplier, &mcginley_bands.1);
    /// assert_eq!((96.81953334480858, 99.53246533805869, 102.2453973313088), mcginley_bands);
    /// ```
    pub fn mcginley_dynamic_bands(
        prices: &[f64],
        deviation_model: &DeviationModel,
        deviation_multiplier: &f64,
        previous_mcginley_dynamic: &f64,
    ) -> (f64, f64, f64) {
        if prices.is_empty() {
            panic!("Prices cannot be empty")
        };

        let last_price = prices.last().unwrap();
        let mcginley_dynamic =
            mcginley_dynamic(&last_price, previous_mcginley_dynamic, &prices.len());

        let deviation = match deviation_model {
            DeviationModel::StandardDeviation => standard_deviation(&prices),
            DeviationModel::MeanAbsoluteDeviation => {
                absolute_deviation(&prices, &crate::CentralPoint::Mean)
            }
            DeviationModel::MedianAbsoluteDeviation => {
                absolute_deviation(&prices, &crate::CentralPoint::Median)
            }
            DeviationModel::ModeAbsoluteDeviation => {
                absolute_deviation(&prices, &crate::CentralPoint::Mode)
            }
            DeviationModel::UlcerIndex => ulcer_index(&prices),
            _ => panic!("Unsupported DeviationModel"),
        };
        let deviation_multiplied = deviation * deviation_multiplier;
        let upper_band = &mcginley_dynamic + &deviation_multiplied;
        let lower_band = &mcginley_dynamic - &deviation_multiplied;
        return (lower_band, mcginley_dynamic, upper_band);
    }

    /// The `ichimoku_cloud` calculates support and resistance levels from past prices.
    ///
    /// The caller determines the conversion, base, and span B period, the standard for these are
    /// 9, 26, and 52 respectively. These were determined when the work day used to be 6
    /// days.
    ///
    /// The function returns the leading span A, leading span B, base line, conversion line, and lagged price
    /// from the beginning of the base period. The simpler Ichimoku clouds focus primarily on
    /// charting span A and B against the candle. More advanced setups add base, conversion
    /// line, and lagged price to show resistance and support levels in more depth.
    ///
    /// # Arguments
    ///
    /// * `highs` - Slice of price highs
    /// * `lows` - Slice of price lows
    /// * `close` - Slice of closing prices
    /// * `conversion_period` - Period used to calculate the conversion line
    /// * `base_period` - Period used to calculate the base line
    /// * `span_b_period` - Period used to calculate the Span B line
    ///
    /// # Panics
    ///
    /// `ichimoku_cloud` will panic if:
    /// * the length of `highs`, `lows`, and `close` don't match
    /// * `conversion_period`, `base_period`, or `span_b_period` are greater than the
    /// length of `highs`, `lows`, and `close`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let high_prices = vec![105.0, 103.0, 107.0, 101.0, 103.0, 100.0, 109.0, 105.0, 110.0, 112.0,
    /// 111.0, 105.0, 106.0, 100.0, 103.0];
    /// let low_prices = vec![97.0, 99.0, 98.0, 100.0, 95.0, 98.0, 99.0, 100.0, 102.0, 106.0,
    /// 99.0, 101.0, 98.0, 93.0, 98.0];
    /// let closing_prices = vec![100.0, 102.0, 103.0, 101.0, 99.0, 99.0, 102.0, 103.0, 106.0, 107.0,
    /// 105.0, 104.0, 101.0, 97.0, 100.0];
    ///
    /// let conversion_period: usize = 5;
    /// let base_period: usize = 10;
    /// let span_b_period: usize = 15;
    ///
    /// let ichimoku_cloud = rust_ti::candle_indicators::single::ichimoku_cloud(&high_prices,
    /// &low_prices, &closing_prices, &conversion_period, &base_period, &span_b_period);
    /// assert_eq!((102.25, 102.5, 102.5, 102.0, 99.0), ichimoku_cloud);
    /// ```
    pub fn ichimoku_cloud(
        highs: &[f64],
        lows: &[f64],
        close: &[f64],
        conversion_period: &usize,
        base_period: &usize,
        span_b_period: &usize,
    ) -> (f64, f64, f64, f64, f64) {
        let length = highs.len();
        if length != lows.len() || length != close.len() {
            panic!(
                "Length of highs ({}) must equal length of lows ({}) and length of close ({})",
                length,
                lows.len(),
                close.len()
            )
        };

        let max_period = conversion_period.max(base_period.max(span_b_period));
        if &length < max_period {
            panic!(
                "Length of prices ({}) cannot be smaller than the size of periods ({})",
                length, max_period,
            );
        };
        let conversion_line = (max(&highs[length - conversion_period..])
            + min(&lows[length - conversion_period..]))
            / 2.0;
        let base_line =
            (max(&highs[length - base_period..]) + min(&lows[length - base_period..])) / 2.0;
        let leading_span_a = (&conversion_line + &base_line) / 2.0;
        let leading_span_b =
            (max(&highs[length - span_b_period..]) + min(&lows[length - span_b_period..])) / 2.0;
        return (
            leading_span_a,
            leading_span_b,
            base_line,
            conversion_line,
            close[length - base_period],
        );
    }

    /// The `donchian_channels` produces bands from the period highs and lows.
    ///
    /// It returns a tuple of the lower band, the middle band, and the upper band.
    ///
    /// # Arguments
    ///
    /// * `high` - Slice of highs
    /// * `low` - Slow of lows
    ///
    /// # Panics
    ///
    /// `donchian_channels` will panic if:
    ///     * lengths of `high` and `low` aren't equal
    ///     * `high` or `low` is empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// let highs = vec![105.0, 103.0, 107.0, 101.0, 103.0];
    /// let lows = vec![97.0, 99.0, 98.0, 100.0, 95.0];
    ///
    /// let donchian_channels = rust_ti::candle_indicators::single::donchian_channels(
    ///     &highs,
    ///     &lows
    /// );
    ///
    /// assert_eq!((95.0, 101.0 ,107.0), donchian_channels);
    /// ```
    pub fn donchian_channels(high: &[f64], low: &[f64]) -> (f64, f64, f64) {
        if high.len() != low.len() {
            panic!("High ({}) must be of same length as low ({})", high.len(), low.len())
        };
        if high.is_empty() {
            panic!("Prices cannot be empty")
        };
        let max_price = max(high);
        let min_price = min(low);
        return (min_price, (max_price + min_price) / 2.0, max_price)
    }

    /// The `keltner_channel` produces bands based on the moving average of prices for
    /// a period and multiplies it by the average true range.
    ///
    /// The standard Keltner Channel uses an exponential moving average and multiplier of 2.
    ///
    /// The function returns a tuple of the lower band, the middle band, and the upper band.
    ///
    /// # Arguments
    ///
    /// * `high` - Slice of highs
    /// * `low` - Slice of lows
    /// * `close` - Slice of previous closing prices. The need to be the closing prices for t-n to
    /// t-1, it cannot be the close from the same day of the high and low.
    /// * `constant_type_model` - Variant of [`ConstantModelType`] for the function
    /// * `atr_constant_type_model` - Variant of [`ConstantModelType`] for the ATR
    /// * `multiplier` - Multiplier for the ATR
    ///
    /// # Panics
    /// 
    /// `keltner_channel` will panic if:
    ///     * Lengths of `high`, `low`, and `close` aren't equal
    ///     * `high`, `low`, or `close` are empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// let highs = vec![105.0, 103.0, 107.0, 101.0, 105.0];
    /// let lows = vec![97.0, 99.0, 98.0, 97.0, 95.0];
    /// let close = vec![101.0, 102.0, 100.0, 99.0, 104.0];
    ///
    /// let keltner_channel = rust_ti::candle_indicators::single::keltner_channel(
    ///     &highs,
    ///     &lows,
    ///     &close,
    ///     &rust_ti::ConstantModelType::ExponentialMovingAverage,
    ///     &rust_ti::ConstantModelType::SimpleMovingAverage,
    ///     &2.0
    /// );
    ///
    /// assert_eq!((86.76777251184836, 100.76777251184836, 114.76777251184836), keltner_channel);
    /// ```
    pub fn keltner_channel(
        high: &[f64],
        low: &[f64],
        close: &[f64],
        constant_model_type: &ConstantModelType,
        atr_constant_model_type: &ConstantModelType,
        multiplier: &f64
    ) -> (f64, f64, f64) {
        let length = high.len();
        if length != low.len() || length != close.len() {
            panic!(
                "Length of high ({}), low ({}), and close ({}) must be equal",
                length, low.len(), close.len()
            )
        };
        if high.is_empty() {
            panic!("Prices cannot be empty")
        };

        let atr = average_true_range(&close, &high, &low, atr_constant_model_type);
        let mut prices = Vec::new();
        for i in 0..length {
            prices.push((high[i]+low[i]+close[i])/3.0);
        };

        let mc = match constant_model_type {
            ConstantModelType::SimpleMovingAverage => {
                moving_average(&prices, &MovingAverageType::Simple)
            }
            ConstantModelType::SmoothedMovingAverage => {
                moving_average(&prices, &MovingAverageType::Smoothed)
            }
            ConstantModelType::ExponentialMovingAverage => {
                moving_average(&prices, &MovingAverageType::Exponential)
            }
            ConstantModelType::PersonalisedMovingAverage(alpha_nominator, alpha_denominator) => {
                moving_average(
                    &prices,
                    &MovingAverageType::Personalised(alpha_nominator, alpha_denominator),
                )
            }
            ConstantModelType::SimpleMovingMedian => median(&prices),
            ConstantModelType::SimpleMovingMode => mode(&prices),
            _ => panic!("Unsupported ConstantModelType"),
        };
        let constant = atr * multiplier;
        return (mc - constant, mc, mc + constant)
    }
}

/// `bulk` holds functions that return multiple values
pub mod bulk {
    use crate::candle_indicators::single;
    /// The `moving_constant_envelopes` function calculates upper and lower bands from the
    /// moving constant of the price. The function returns a tuple with the lower band,
    /// moving constant, upper band (in that order).
    ///
    /// The standard is to use the moving averages to create a moving average envelope but this
    /// function has been extended to allow the caller to use the other constant model types, hence
    /// the name.
    ///
    /// The caller also determines the difference, or the distance between the price and the bands.
    /// The `int` passed in is a percentage, so passing in 3 would mean a 3% difference from the
    /// moving average.
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `constant_model_type` - Variant of the [`ConstantModelType`](crate::ConstantModelType) enum.
    /// * `difference` - The percent difference or distance that the bands will be from the moving
    /// constant
    /// * `period` - Period over which to calculate the moving constant envelopes
    ///
    /// # Panics
    ///
    /// `moving_constant_envelopes` will panic if `period` is larger than length of `prices`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0, 99.0, 102.0];
    /// let difference = 3.0;
    /// let period: usize = 5;
    ///
    /// let ema_envelope = rust_ti::candle_indicators::bulk::moving_constant_envelopes(&prices,
    /// &rust_ti::ConstantModelType::ExponentialMovingAverage, &difference, &period);
    /// assert_eq!(vec![(97.59303317535547, 100.61137440758296, 103.62971563981044), (97.02298578199054, 100.02369668246448, 103.02440758293841), (97.66199052132701, 100.6824644549763, 103.70293838862558)], ema_envelope);
    ///
    /// let median_envelope = rust_ti::candle_indicators::bulk::moving_constant_envelopes(&prices,
    /// &rust_ti::ConstantModelType::SimpleMovingMedian, &difference, &period);
    /// assert_eq!(vec![(97.97, 101.0, 104.03), (97.97, 101.0, 104.03), (97.97, 101.0, 104.03)], median_envelope);
    /// ```
    pub fn moving_constant_envelopes(
        prices: &[f64],
        constant_model_type: &crate::ConstantModelType,
        difference: &f64,
        period: &usize,
    ) -> Vec<(f64, f64, f64)> {
        let length = prices.len();
        if period > &length {
            panic!(
                "Period ({}) cannot be greater than length of prices ({})",
                period, length
            );
        };

        let mut moving_envelopes = Vec::new();
        let loop_max = length - period + 1;
        for i in 0..loop_max {
            moving_envelopes.push(single::moving_constant_envelopes(
                &prices[i..i + period],
                constant_model_type,
                difference,
            ));
        }
        return moving_envelopes;
    }

    /// The `mcginley_dynamic_envelopes` function calculates upper and lower bands from the
    /// Mcginley dynamic of the price. The function returns a tuple with the lower band,
    /// McGinley dynamic, upper band (in that order).
    ///
    /// This is a variation of the `moving_constant_envelopes` function.
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `difference` - The percent difference or distance that the bands will be from the moving
    /// constant
    /// * `previous_mcginley_dynamic` - Previous value for the McGinley dynamic. 0.0 is no
    /// previous.
    /// * `period` - Period over which to calculate the McGinley dynamic envelopes.
    ///
    /// # Panics
    ///
    /// `mcginley_dynamic_envelopes` will panic if `period` is larger than length of `prices`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0, 99.0, 102.0];
    /// let difference = 3.0;
    ///
    /// let mcginley_envelope = rust_ti::candle_indicators::bulk::mcginley_dynamic_envelopes(&prices,
    ///  &difference, &0.0, &5_usize);
    /// assert_eq!(vec![(96.03, 99.0, 101.97), (96.03, 99.0, 101.97), (96.54649137791692, 99.53246533805869, 102.51843929820045)], mcginley_envelope);
    /// ```
    pub fn mcginley_dynamic_envelopes(
        prices: &[f64],
        difference: &f64,
        previous_mcginley_dynamic: &f64,
        period: &usize,
    ) -> Vec<(f64, f64, f64)> {
        let length = prices.len();
        if period > &length {
            panic!(
                "Period ({}) cannot be longer the length of prices ({})",
                period, length
            );
        };

        let mut mcginley_envelopes = vec![single::mcginley_dynamic_envelopes(
            &prices[..*period],
            difference,
            previous_mcginley_dynamic,
        )];
        let loop_max = length - period + 1;
        for i in 1..loop_max {
            let previous_dynamic = mcginley_envelopes[i - 1].1;
            mcginley_envelopes.push(single::mcginley_dynamic_envelopes(
                &prices[i..i + period],
                difference,
                &previous_dynamic,
            ));
        }
        return mcginley_envelopes;
    }

    /// The `moving_constant_bands` are inspired from the Bollinger Bands but have been extended to
    /// allow the caller to determine the moving constant model, the deviation model, and the
    /// multiplier of the deviation model.
    ///
    /// Returns a vector of tuples with the lower band, moving constant, the upper band
    ///
    /// The standard for Bollinger Bands is to use a moving average model, the standard deviation,
    /// and a standard deviation model of 2.
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `constant_model_type` - Variant of the [`ConstantModelType`](crate::ConstantModelType) enum.
    /// * `deviation_model` - Variant of the [`DeviationModel`](crate::DeviationModel) enum.
    /// * `deviation_multiplier` - Multiplier for the deviation of prices.
    /// * `period` - Period over which to calculate the moving constant bands.
    ///
    /// # Panics
    ///
    /// `moving_constant_bands` panics if `period` is greater than length of `prices`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0, 99.0, 102.0];
    /// let multiplier = 2.0;
    /// let period: usize = 5;
    ///
    /// let bollinger_bands = rust_ti::candle_indicators::bulk::moving_constant_bands(&prices,
    /// &rust_ti::ConstantModelType::SimpleMovingAverage, &rust_ti::DeviationModel::StandardDeviation, &multiplier, &period);
    /// assert_eq!(vec![(98.17157287525382, 101.0, 103.82842712474618), (97.6, 100.8, 104.0), (97.6, 100.8, 104.0)], bollinger_bands);
    ///
    /// let ema_mad_bands = rust_ti::candle_indicators::bulk::moving_constant_bands(&prices, &rust_ti::ConstantModelType::ExponentialMovingAverage, &rust_ti::DeviationModel::MeanAbsoluteDeviation, &multiplier, &period);
    /// assert_eq!(vec![(98.21137440758295, 100.61137440758296, 103.01137440758296), (97.14369668246448, 100.02369668246448, 102.90369668246447), (97.8024644549763, 100.6824644549763, 103.5624644549763)], ema_mad_bands);
    /// ```
    pub fn moving_constant_bands(
        prices: &[f64],
        constant_model_type: &crate::ConstantModelType,
        deviation_model: &crate::DeviationModel,
        deviation_multiplier: &f64,
        period: &usize,
    ) -> Vec<(f64, f64, f64)> {
        let length = prices.len();
        if period > &length {
            panic!(
                "Period ({}) cannot be longer then length of prices ({})",
                period, length
            )
        };

        let mut constant_bands = Vec::new();
        let loop_max = length - period + 1;
        for i in 0..loop_max {
            constant_bands.push(single::moving_constant_bands(
                &prices[i..i + period],
                constant_model_type,
                deviation_model,
                deviation_multiplier,
            ));
        }
        return constant_bands;
    }

    /// The `mcginley_dynamic_bands` are a variation of the `moving_constant_bands` but uses
    /// the McGinley Dynamic rather than a moving constant model.
    ///
    /// Returns a tuple with the lower band, McGinley dynamic, the upper band
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `deviation_model` - Variant of the [`DeviationModel`](crate::DeviationModel) enum.
    /// * `deviation_multiplier` - Multiplier for the deviation of prices.
    /// * `previous_mcginley_dynamic` - Previous value for the McGinley dynamic. 0.0 if no
    /// previous.
    /// * `period` - Period over which to calculate the McGinley dynamic bands.
    ///
    /// # Panics
    ///
    /// `mcginley_dynamic_bands` panics if `period` is greater than length of `prices`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0, 99.0, 102.0];
    /// let multiplier = 2.0;
    /// let period: usize = 5;
    ///
    /// let mcginley_bands = rust_ti::candle_indicators::bulk::mcginley_dynamic_bands(&prices,
    /// &rust_ti::DeviationModel::StandardDeviation, &multiplier, &0.0, &period);
    /// assert_eq!(vec![(96.17157287525382, 99.0, 101.82842712474618), (95.8, 99.0, 102.2),
    /// (96.33246533805868, 99.53246533805869, 102.73246533805869)], mcginley_bands);
    /// ```
    pub fn mcginley_dynamic_bands(
        prices: &[f64],
        deviation_model: &crate::DeviationModel,
        deviation_multiplier: &f64,
        previous_mcginley_dynamic: &f64,
        period: &usize,
    ) -> Vec<(f64, f64, f64)> {
        let length = prices.len();
        if period > &length {
            panic!(
                "Period ({}) cannot be longer then length of prices ({})",
                period, length
            )
        };

        let mut mcginley_bands = vec![single::mcginley_dynamic_bands(
            &prices[..*period],
            deviation_model,
            deviation_multiplier,
            previous_mcginley_dynamic,
        )];
        let loop_max = length - period + 1;
        for i in 1..loop_max {
            let previous_dynamic = mcginley_bands[i - 1].1;
            mcginley_bands.push(single::mcginley_dynamic_bands(
                &prices[i..i + period],
                deviation_model,
                deviation_multiplier,
                &previous_dynamic,
            ));
        }
        return mcginley_bands;
    }

    /// The `ichimoku_cloud` attempts to calculates support and resistance levels from past prices.
    ///
    /// The caller determines the conversion, base, and span B period, the standard for these are
    /// 9, 26, and 52 respectively, however these were determined when the work day used to be 6
    /// days.
    ///
    /// The function returns the leading span A, leading span B, base, conversion line, and price
    /// from the beginning of the base period. The simpler Ichimoku clouds focus primarily on
    /// charting span A and B against the candle. More advanced setups add base, conversion
    /// line, and lagged price to show resistance and support levels in more depth.
    ///
    /// # Arguments
    ///
    /// * `highs` - Slice of price highs
    /// * `lows` - Slice of price lows
    /// * `close` - Slice of closing prices
    /// * `conversion_period` - Period used to calculate the conversion line
    /// * `base_period` - Period used to calculate the base line
    /// * `span_b_period` - Period used to calculate the Span B line
    ///
    /// # Panics
    ///
    /// `ichimoku_cloud` will panic if:
    /// * the length of `highs`, `lows`, and `close` don't match
    /// * `conversion_period`, `base_period`, or `span_b_period` are greater than the
    /// length of `highs`, `lows`, and `close`

    ///
    /// # Examples
    ///
    /// ```rust
    /// let high_prices = vec![105.0, 103.0, 107.0, 101.0, 103.0, 100.0, 109.0, 105.0, 110.0, 112.0,
    /// 111.0, 105.0, 106.0, 100.0, 103.0, 102.0, 98.0];
    /// let low_prices = vec![97.0, 99.0, 98.0, 100.0, 95.0, 98.0, 99.0, 100.0, 102.0, 106.0,
    /// 99.0, 101.0, 98.0, 93.0, 98.0, 91.0, 89.0];
    /// let closing_prices = vec![100.0, 102.0, 103.0, 101.0, 99.0, 99.0, 102.0, 103.0, 106.0, 107.0,
    /// 105.0, 104.0, 101.0, 97.0, 100.0, 96.0, 93.0];
    ///
    /// let conversion_period: usize = 5;
    /// let base_period: usize = 10;
    /// let span_b_period: usize = 15;
    ///
    /// let ichimoku_cloud = rust_ti::candle_indicators::bulk::ichimoku_cloud(&high_prices,
    /// &low_prices, &closing_prices, &conversion_period, &base_period, &span_b_period);
    /// assert_eq!(vec![(102.25, 102.5, 102.5, 102.0, 99.0), (100.0, 101.5, 101.5, 98.5, 102.0), (99.0, 100.5, 100.5, 97.5, 103.0)], ichimoku_cloud);
    /// ```
    pub fn ichimoku_cloud(
        highs: &[f64],
        lows: &[f64],
        close: &[f64],
        conversion_period: &usize,
        base_period: &usize,
        span_b_period: &usize,
    ) -> Vec<(f64, f64, f64, f64, f64)> {
        let length = highs.len();
        if length != lows.len() || length != close.len() {
            panic!(
                "Length of highs ({}) must equal length of lows ({}) and length of close ({})",
                length,
                lows.len(),
                close.len()
            )
        };

        let max_period = conversion_period.max(base_period.max(span_b_period));
        if &length < max_period {
            panic!(
                "Length of prices ({}) cannot be smaller than the size of periods ({})",
                length, max_period,
            );
        };
        let mut ichimoku_clouds = Vec::new();
        let loop_max = length - max_period + 1;
        for i in 0..loop_max {
            ichimoku_clouds.push(single::ichimoku_cloud(
                &highs[i..i + max_period],
                &lows[i..i + max_period],
                &close[i..i + max_period],
                conversion_period,
                base_period,
                span_b_period,
            ));
        }
        return ichimoku_clouds;
    }

    /// The `donchian_channels` produces bands from the period high and low.
    ///
    /// It returns a vector with tuples of the lower band, the middle band, and the upper band.
    ///
    /// # Arguments
    ///
    /// * `high` - Slice of highs
    /// * `low` - Slow of lows
    /// * `period` - Period over which to calculate the Donchian channels
    ///
    /// # Panics
    ///
    /// `donchian_channels` will panic if:
    ///     * lengths of `high` and `low` aren't equal
    ///     * `high` or `low` is empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// let highs = vec![105.0, 103.0, 107.0, 101.0, 103.0, 100.0, 109.0, 105.0];
    /// let lows = vec![97.0, 99.0, 98.0, 100.0, 95.0, 98.0, 99.0, 100.0];
    /// let period: usize = 5;
    ///
    /// let donchian_channels = rust_ti::candle_indicators::bulk::donchian_channels(
    ///     &highs,
    ///     &lows,
    ///     &period
    /// );
    ///
    /// assert_eq!(vec![
    ///         (95.0, 101.0, 107.0), (95.0, 101.0, 107.0), (95.0, 102.0, 109.0), (95.0, 102.0, 109.0)
    ///     ], donchian_channels);
    /// ```
    pub fn donchian_channels(high: &[f64], low: &[f64], period: &usize) -> Vec<(f64, f64, f64)> {
        let length = high.len();
        if length != low.len() {
            panic!("High ({}) must be of same length as low ({})", length, low.len())
        };
        if high.is_empty() {
            panic!("Prices cannot be empty")
        };
        if period > &length {
            panic!("Period ({}) must be less than or equal to length of price ({})", period, length)
        };
        let mut dcs = Vec::new();
        let loop_max = length - period + 1;
        for i in 0..loop_max {
            dcs.push(single::donchian_channels(&high[i..i+period], &low[i..i+period]));
        };
        return dcs;
    }

    /// The `keltner_channel` produces bands based on the moving average of prices for
    /// a period and multiplies it by the average true range.
    ///
    /// The standard Keltner Channel uses an exponential moving average and multiplier of 2.
    ///
    /// The function returns a tuple of the lower band, the middle band, and the upper band.
    ///
    /// # Arguments
    ///
    /// * `high` - Slice of highs
    /// * `low` - Slice of lows
    /// * `close` - Slice of previous closing prices. The need to be the closing prices for t-n to
    /// t-1, it cannot be the close from the same day of the high and low.
    /// * `constant_type_model` - Variant of [`ConstantModelType`] for the function
    /// * `atr_constant_type_model` - Variant of [`ConstantModelType`] for the ATR
    /// * `multiplier` - Multiplier for the ATR
    /// * `period` - Period over which to calculate the Keltner Channel
    ///
    /// # Panics
    /// 
    /// `keltner_channel` will panic if:
    ///     * Lengths of `high`, `low`, and `close` aren't equal
    ///     * `high`, `low`, or `close` are empty
    ///     * If `period` is greater than lengths
    ///
    /// # Examples
    ///
    /// ```rust
    /// let highs = vec![105.0, 103.0, 107.0, 101.0, 105.0, 109.0, 111.0];
    /// let lows = vec![97.0, 99.0, 98.0, 97.0, 95.0, 102.0, 106.0];
    /// let close = vec![101.0, 102.0, 100.0, 99.0, 104.0, 107.0, 108.0];
    /// let period: usize = 5;
    ///
    /// let keltner_channel = rust_ti::candle_indicators::bulk::keltner_channel(
    ///     &highs,
    ///     &lows,
    ///     &close,
    ///     &rust_ti::ConstantModelType::ExponentialMovingAverage,
    ///     &rust_ti::ConstantModelType::SimpleMovingAverage,
    ///     &2.0,
    ///     &period
    /// );
    ///
    /// assert_eq!(
    ///     vec![
    ///         (86.76777251184836, 100.76777251184836, 114.76777251184836), 
    ///         (89.16461295418642, 102.76461295418642, 116.36461295418641),
    ///         (90.9747235387046, 104.9747235387046, 118.9747235387046)
    ///     ], keltner_channel);
    /// ```
    pub fn keltner_channel(
        high: &[f64],
        low: &[f64],
        close: &[f64],
        constant_model_type: &crate::ConstantModelType,
        atr_constant_model_type: &crate::ConstantModelType,
        multiplier: &f64,
        period: &usize
    ) -> Vec<(f64, f64, f64)> {
        let length = high.len();
        if length != low.len() || length != close.len() {
            panic!(
                "Length of high ({}), low ({}), and close ({}) must be equal",
                length, low.len(), close.len()
            )
        };
        if high.is_empty() {
            panic!("Prices cannot be empty")
        };
        if period > &length {
            panic!("Period ({}) cannot be greater than length of prices ({})", period, length)
        };

        let mut kcs = Vec::new();
        let loop_max = length - period + 1;
        for i in 0..loop_max {
            kcs.push(single::keltner_channel(
                    &high[i..i+period],
                    &low[i..i+period],
                    &close[i..i+period],
                    constant_model_type,
                    atr_constant_model_type,
                    multiplier
            ));
        };
        return kcs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_ma_moving_constant_envelope() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            (97.34338, 100.354, 103.36462),
            single::moving_constant_envelopes(
                &prices,
                &crate::ConstantModelType::SimpleMovingAverage,
                &3.0
            )
        );
    }

    #[test]
    fn test_single_sma_moving_constant_envelope() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            (97.30730209424084, 100.31680628272251, 103.32631047120418),
            single::moving_constant_envelopes(
                &prices,
                &crate::ConstantModelType::SmoothedMovingAverage,
                &3.0
            )
        );
    }

    #[test]
    fn test_single_ema_moving_constant_envelope() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            (97.28056445497631, 100.28924170616115, 103.29791895734598),
            single::moving_constant_envelopes(
                &prices,
                &crate::ConstantModelType::ExponentialMovingAverage,
                &3.0
            )
        );
    }

    #[test]
    fn test_single_pma_moving_constant_envelope() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            (97.2379964584231, 100.24535717363206, 103.25271788884102),
            single::moving_constant_envelopes(
                &prices,
                &crate::ConstantModelType::PersonalisedMovingAverage(&5.0, &4.0),
                &3.0
            )
        );
    }

    #[test]
    fn test_single_median_moving_constant_envelope() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            (97.36859999999999, 100.38, 103.3914),
            single::moving_constant_envelopes(
                &prices,
                &crate::ConstantModelType::SimpleMovingMedian,
                &3.0
            )
        );
    }

    #[test]
    fn test_single_mode_moving_constant_envelope() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            (97.0, 100.0, 103.0),
            single::moving_constant_envelopes(
                &prices,
                &crate::ConstantModelType::SimpleMovingMode,
                &3.0
            )
        );
    }

    #[test]
    #[should_panic]
    fn test_single_moving_constant_envelope_panic() {
        let prices = Vec::new();
        single::moving_constant_envelopes(
            &prices,
            &crate::ConstantModelType::SimpleMovingMode,
            &3.0,
        );
    }

    #[test]
    fn test_bulk_moving_constant_envelope() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        assert_eq!(
            vec![
                (97.28056445497631, 100.28924170616115, 103.29791895734598),
                (97.28364454976304, 100.29241706161139, 103.30118957345974),
                (97.26737061611377, 100.27563981042657, 103.28390900473937)
            ],
            bulk::moving_constant_envelopes(
                &prices,
                &crate::ConstantModelType::ExponentialMovingAverage,
                &3.0,
                &5_usize
            )
        );
    }

    #[test]
    #[should_panic]
    fn test_bulk_moving_constant_envelope_panic() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        bulk::moving_constant_envelopes(
            &prices,
            &crate::ConstantModelType::ExponentialMovingAverage,
            &3.0,
            &50_usize,
        );
    }

    #[test]
    fn test_single_mcginley_envelope_no_previous() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            (97.2037, 100.21, 103.21629999999999),
            single::mcginley_dynamic_envelopes(&prices, &3.0, &0.0)
        );
    }

    #[test]
    fn test_single_mcginley_envelope_previous() {
        let prices = vec![100.53, 100.38, 100.19, 100.21, 100.32];
        assert_eq!(
            (97.22494655733786, 100.23190366735862, 103.23886077737939),
            single::mcginley_dynamic_envelopes(&prices, &3.0, &100.21)
        );
    }

    #[test]
    #[should_panic]
    fn test_single_mcginley_envelope_panic() {
        let prices = Vec::new();
        single::mcginley_dynamic_envelopes(&prices, &3.0, &0.0);
    }

    #[test]
    fn test_bulk_mcginley_envelope_no_previous() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        assert_eq!(
            vec![
                (97.2037, 100.21, 103.21629999999999),
                (97.22494655733786, 100.23190366735862, 103.23886077737939),
                (97.23425935799065, 100.24150449277387, 103.24874962755709)
            ],
            bulk::mcginley_dynamic_envelopes(&prices, &3.0, &0.0, &5_usize)
        );
    }

    #[test]
    fn test_bulk_mcginley_envelope_previous() {
        let prices = vec![100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        assert_eq!(
            vec![
                (97.22494655733786, 100.23190366735862, 103.23886077737939),
                (97.23425935799065, 100.24150449277387, 103.24874962755709)
            ],
            bulk::mcginley_dynamic_envelopes(&prices, &3.0, &100.21, &5_usize)
        );
    }

    #[test]
    #[should_panic]
    fn test_bulk_mcginley_envelope_panic() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        bulk::mcginley_dynamic_envelopes(&prices, &3.0, &0.0, &50_usize);
    }

    #[test]
    fn test_single_ma_stddev_constant_bands() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            (100.08489778893514, 100.354, 100.62310221106486),
            single::moving_constant_bands(
                &prices,
                &crate::ConstantModelType::SimpleMovingAverage,
                &crate::DeviationModel::StandardDeviation,
                &2.0
            )
        );
    }

    #[test]
    fn test_single_sma_stddev_constant_bands() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            (100.04770407165765, 100.31680628272251, 100.58590849378737),
            single::moving_constant_bands(
                &prices,
                &crate::ConstantModelType::SmoothedMovingAverage,
                &crate::DeviationModel::StandardDeviation,
                &2.0
            )
        );
    }

    #[test]
    fn test_single_ema_stddev_constant_bands() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            (100.02013949509629, 100.28924170616115, 100.55834391722601),
            single::moving_constant_bands(
                &prices,
                &crate::ConstantModelType::ExponentialMovingAverage,
                &crate::DeviationModel::StandardDeviation,
                &2.0
            )
        );
    }

    #[test]
    fn test_single_pma_stddev_constant_bands() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            (99.9762549625672, 100.24535717363206, 100.51445938469692),
            single::moving_constant_bands(
                &prices,
                &crate::ConstantModelType::PersonalisedMovingAverage(&5.0, &4.0),
                &crate::DeviationModel::StandardDeviation,
                &2.0
            )
        );
    }

    #[test]
    fn test_single_median_stddev_constant_bands() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            (100.11089778893513, 100.38, 100.64910221106486),
            single::moving_constant_bands(
                &prices,
                &crate::ConstantModelType::SimpleMovingMedian,
                &crate::DeviationModel::StandardDeviation,
                &2.0
            )
        );
    }

    #[test]
    fn test_single_mode_stddev_constant_bands() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            (99.73089778893514, 100.0, 100.26910221106486),
            single::moving_constant_bands(
                &prices,
                &crate::ConstantModelType::SimpleMovingMode,
                &crate::DeviationModel::StandardDeviation,
                &2.0
            )
        );
    }

    #[test]
    fn test_single_ma_mean_ad_constant_bands() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            (100.1076, 100.354, 100.6004),
            single::moving_constant_bands(
                &prices,
                &crate::ConstantModelType::SimpleMovingAverage,
                &crate::DeviationModel::MeanAbsoluteDeviation,
                &2.0
            )
        );
    }

    #[test]
    fn test_single_ma_median_ad_constant_bands() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            (100.118, 100.354, 100.59),
            single::moving_constant_bands(
                &prices,
                &crate::ConstantModelType::SimpleMovingAverage,
                &crate::DeviationModel::MedianAbsoluteDeviation,
                &2.0
            )
        );
    }

    #[test]
    fn test_single_ma_mode_ad_constant_bands() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            (99.646, 100.354, 101.062),
            single::moving_constant_bands(
                &prices,
                &crate::ConstantModelType::SimpleMovingAverage,
                &crate::DeviationModel::ModeAbsoluteDeviation,
                &2.0
            )
        );
    }

    #[test]
    fn test_single_ma_ulcer_index_constant_bands() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            (99.91767826122627, 100.354, 100.79032173877373),
            single::moving_constant_bands(
                &prices,
                &crate::ConstantModelType::SimpleMovingAverage,
                &crate::DeviationModel::UlcerIndex,
                &2.0
            )
        );
    }

    #[test]
    #[should_panic]
    fn test_single_constant_bands_panic() {
        let prices = Vec::new();
        single::moving_constant_bands(
            &prices,
            &crate::ConstantModelType::SimpleMovingAverage,
            &crate::DeviationModel::ModeAbsoluteDeviation,
            &2.0,
        );
    }

    #[test]
    fn test_bulk_constant_bands() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        assert_eq!(
            vec![
                (100.08489778893514, 100.354, 100.62310221106486),
                (100.07858132649292, 100.326, 100.57341867350706),
                (100.1359428687999, 100.276, 100.41605713120009)
            ],
            bulk::moving_constant_bands(
                &prices,
                &crate::ConstantModelType::SimpleMovingAverage,
                &crate::DeviationModel::StandardDeviation,
                &2.0,
                &5_usize
            )
        );
    }

    #[test]
    #[should_panic]
    fn test_bulk_constant_bands_panic() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        bulk::moving_constant_bands(
            &prices,
            &crate::ConstantModelType::SimpleMovingAverage,
            &crate::DeviationModel::StandardDeviation,
            &2.0,
            &50_usize,
        );
    }

    #[test]
    fn test_single_mcginley_bands_std_dev_no_previous() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            (99.94089778893513, 100.21, 100.47910221106486),
            single::mcginley_dynamic_bands(
                &prices,
                &crate::DeviationModel::StandardDeviation,
                &2.0,
                &0.0
            )
        );
    }

    #[test]
    fn test_single_mcginley_bands_mean_ad_no_previous() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            (99.9636, 100.21, 100.45639999999999),
            single::mcginley_dynamic_bands(
                &prices,
                &crate::DeviationModel::MeanAbsoluteDeviation,
                &2.0,
                &0.0
            )
        );
    }

    #[test]
    fn test_single_mcginley_bands_median_ad_no_previous() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            (99.97399999999999, 100.21, 100.446),
            single::mcginley_dynamic_bands(
                &prices,
                &crate::DeviationModel::MedianAbsoluteDeviation,
                &2.0,
                &0.0
            )
        );
    }

    #[test]
    fn test_single_mcginley_bands_mode_ad_no_previous() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            (99.502, 100.21, 100.91799999999999),
            single::mcginley_dynamic_bands(
                &prices,
                &crate::DeviationModel::ModeAbsoluteDeviation,
                &2.0,
                &0.0
            )
        );
    }

    #[test]
    fn test_single_mcginley_bands_ulcer_index_no_previous() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            (99.77367826122627, 100.21, 100.64632173877372),
            single::mcginley_dynamic_bands(&prices, &crate::DeviationModel::UlcerIndex, &2.0, &0.0)
        );
    }

    #[test]
    fn test_single_mcginley_bands_std_dev_previous() {
        let prices = vec![100.53, 100.38, 100.19, 100.21, 100.32];
        assert_eq!(
            (99.98448499385155, 100.23190366735862, 100.47932234086569),
            single::mcginley_dynamic_bands(
                &prices,
                &crate::DeviationModel::StandardDeviation,
                &2.0,
                &100.21
            )
        );
    }

    #[test]
    #[should_panic]
    fn test_sinlge_mcginley_bands_panic() {
        let prices = Vec::new();
        single::mcginley_dynamic_bands(
            &prices,
            &crate::DeviationModel::StandardDeviation,
            &2.0,
            &0.0,
        );
    }

    #[test]
    fn test_bulk_mcginley_bands_no_previous() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        assert_eq!(
            vec![
                (99.94089778893513, 100.21, 100.47910221106486),
                (99.98448499385155, 100.23190366735862, 100.47932234086569),
                (100.10144736157378, 100.24150449277387, 100.38156162397397)
            ],
            bulk::mcginley_dynamic_bands(
                &prices,
                &crate::DeviationModel::StandardDeviation,
                &2.0,
                &0.0,
                &5_usize
            )
        );
    }

    #[test]
    fn test_bulk_mcginley_bands_previous() {
        let prices = vec![100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        assert_eq!(
            vec![
                (99.98448499385155, 100.23190366735862, 100.47932234086569),
                (100.10144736157378, 100.24150449277387, 100.38156162397397)
            ],
            bulk::mcginley_dynamic_bands(
                &prices,
                &crate::DeviationModel::StandardDeviation,
                &2.0,
                &100.21,
                &5_usize
            )
        );
    }

    #[test]
    #[should_panic]
    fn test_bulk_mcginley_bands_panic() {
        let prices = vec![100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        bulk::mcginley_dynamic_bands(
            &prices,
            &crate::DeviationModel::StandardDeviation,
            &2.0,
            &100.21,
            &50_usize,
        );
    }

    #[test]
    fn test_single_ichimoku_cloud() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83, 101.73, 102.01];
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        let close = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        assert_eq!(
            (100.595, 100.66, 100.65, 100.53999999999999, 100.38),
            single::ichimoku_cloud(&highs, &lows, &close, &3_usize, &5_usize, &7_usize)
        );
    }

    #[test]
    #[should_panic]
    fn test_single_ichimoku_high_size_panic() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83, 101.73];
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        let close = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        single::ichimoku_cloud(&highs, &lows, &close, &3_usize, &5_usize, &7_usize);
    }

    #[test]
    #[should_panic]
    fn test_single_ichimoku_low_size_panic() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83, 101.73, 102.01];
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1];
        let close = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        single::ichimoku_cloud(&highs, &lows, &close, &3_usize, &5_usize, &7_usize);
    }

    #[test]
    #[should_panic]
    fn test_single_ichimoku_close_size_panic() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83, 101.73, 102.01];
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        let close = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32];
        single::ichimoku_cloud(&highs, &lows, &close, &3_usize, &5_usize, &7_usize);
    }

    #[test]
    #[should_panic]
    fn test_single_ichimoku_conversion_panic() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83, 101.73, 102.01];
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        let close = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        single::ichimoku_cloud(&highs, &lows, &close, &30_usize, &5_usize, &7_usize);
    }

    #[test]
    #[should_panic]
    fn test_single_ichimoku_base_panic() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83, 101.73, 102.01];
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        let close = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        single::ichimoku_cloud(&highs, &lows, &close, &3_usize, &50_usize, &7_usize);
    }

    #[test]
    #[should_panic]
    fn test_single_ichimoku_span_b_panic() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83, 101.73, 102.01];
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        let close = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        single::ichimoku_cloud(&highs, &lows, &close, &3_usize, &5_usize, &70_usize);
    }

    #[test]
    fn test_bulk_ichimoku_clud() {
        let highs = vec![
            101.26, 102.57, 102.32, 100.69, 100.83, 101.73, 102.01, 101.11, 100.75,
        ];
        let lows = vec![
            100.08, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96, 100.21, 100.48,
        ];
        let close = vec![
            100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28, 100.49, 100.52,
        ];
        assert_eq!(
            vec![
                (100.595, 100.66, 100.65, 100.53999999999999, 100.38),
                (100.74000000000001, 100.66, 100.495, 100.985, 100.19),
                (
                    100.76249999999999,
                    100.65,
                    100.53999999999999,
                    100.985,
                    100.21
                )
            ],
            bulk::ichimoku_cloud(&highs, &lows, &close, &3_usize, &5_usize, &7_usize)
        );
    }

    #[test]
    #[should_panic]
    fn test_bulk_ichimoku_high_size_panic() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83, 101.73];
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        let close = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        bulk::ichimoku_cloud(&highs, &lows, &close, &3_usize, &5_usize, &7_usize);
    }

    #[test]
    #[should_panic]
    fn test_bulk_ichimoku_low_size_panic() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83, 101.73, 102.01];
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1];
        let close = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        bulk::ichimoku_cloud(&highs, &lows, &close, &3_usize, &5_usize, &7_usize);
    }

    #[test]
    #[should_panic]
    fn test_bulk_ichimoku_close_size_panic() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83, 101.73, 102.01];
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        let close = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32];
        bulk::ichimoku_cloud(&highs, &lows, &close, &3_usize, &5_usize, &7_usize);
    }

    #[test]
    #[should_panic]
    fn test_bulk_ichimoku_conversion_panic() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83, 101.73, 102.01];
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        let close = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        bulk::ichimoku_cloud(&highs, &lows, &close, &30_usize, &5_usize, &7_usize);
    }

    #[test]
    #[should_panic]
    fn bulk_single_ichimoku_base_panic() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83, 101.73, 102.01];
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        let close = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        bulk::ichimoku_cloud(&highs, &lows, &close, &3_usize, &50_usize, &7_usize);
    }

    #[test]
    #[should_panic]
    fn test_bulk_ichimoku_span_b_panic() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83, 101.73, 102.01];
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        let close = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        bulk::ichimoku_cloud(&highs, &lows, &close, &3_usize, &5_usize, &70_usize);
    }

    #[test]
    fn single_donchian_channel() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83];
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07];
        assert_eq!((98.75, 100.66, 102.57), single::donchian_channels(&highs, &lows));
    }

    #[test]
    #[should_panic]
    fn single_donchian_channel_length_panic() {
        let highs = vec![101.26, 102.57, 100.69, 100.83];
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07];
        single::donchian_channels(&highs, &lows);
    }

    #[test]
    #[should_panic]
    fn single_donchian_channel_empty_panic() {
        let highs = Vec::new();
        let lows = Vec::new();
        single::donchian_channels(&highs, &lows);
    }

    #[test]
    fn bulk_donchian_channels() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83, 101.73, 102.01];
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        assert_eq!(
            vec![
                (98.75, 100.66, 102.57), (98.75, 100.66, 102.57), (98.98, 100.65, 102.32)
            ], bulk::donchian_channels(&highs, &lows, &5_usize)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_donchian_channels_panic_length() {
        let highs = vec![101.26, 102.57, 100.69, 100.83, 101.73, 102.01];
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        bulk::donchian_channels(&highs, &lows, &5_usize);
    }

    #[test]
    #[should_panic]
    fn bulk_donchian_channels_panic_period() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83, 101.73, 102.01];
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        bulk::donchian_channels(&highs, &lows, &50_usize);
    }

    #[test]
    #[should_panic]
    fn bulk_donchian_channels_panic_empty() {
        let highs = Vec::new();
        let lows = Vec::new();
        bulk::donchian_channels(&highs, &lows, &5_usize);
    }

    #[test]
    fn single_keltner_channel_ma() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83];
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07];
        let close = vec![100.94, 101.27, 100.55, 99.01, 100.43];
        assert_eq!((96.19933333333334, 100.45933333333333, 104.71933333333332), 
            single::keltner_channel(
                &highs,
                &lows,
                &close,
                &crate::ConstantModelType::SimpleMovingAverage,
                &crate::ConstantModelType::SimpleMovingAverage,
                &2.0
            ));
    }

    #[test]
    fn single_keltner_channel_sma() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83];
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07];
        let close = vec![100.94, 101.27, 100.55, 99.01, 100.43];
        assert_eq!((96.08312708234176, 100.34312708234175, 104.60312708234174), 
            single::keltner_channel(
                &highs,
                &lows,
                &close,
                &crate::ConstantModelType::SmoothedMovingAverage,
                &crate::ConstantModelType::SimpleMovingAverage,
                &2.0
            ));
    }

    #[test]
    fn single_keltner_channel_ema() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83];
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07];
        let close = vec![100.94, 101.27, 100.55, 99.01, 100.43];
        assert_eq!((95.99663507109007, 100.25663507109006, 104.51663507109005), 
            single::keltner_channel(
                &highs,
                &lows,
                &close,
                &crate::ConstantModelType::ExponentialMovingAverage,
                &crate::ConstantModelType::SimpleMovingAverage,
                &2.0
            ));
    }

    #[test]
    fn single_keltner_channel_pma() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83];
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07];
        let close = vec![100.94, 101.27, 100.55, 99.01, 100.43];
        assert_eq!((95.86329426971132, 100.12329426971131, 104.3832942697113), 
            single::keltner_channel(
                &highs,
                &lows,
                &close,
                &crate::ConstantModelType::PersonalisedMovingAverage(&5.0, &4.0),
                &crate::ConstantModelType::SimpleMovingAverage,
                &2.0
            ));
    }

    #[test]
    fn single_keltner_channel_median() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83];
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07];
        let close = vec![100.94, 101.27, 100.55, 99.01, 100.43];
        assert_eq!((96.5, 100.75999999999999, 105.01999999999998), 
            single::keltner_channel(
                &highs,
                &lows,
                &close,
                &crate::ConstantModelType::SimpleMovingMedian,
                &crate::ConstantModelType::SimpleMovingAverage,
                &2.0
            ));
    }

    #[test]
    fn single_keltner_channel_mode() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83];
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07];
        let close = vec![100.94, 101.27, 100.55, 99.01, 100.43];
        assert_eq!((96.74000000000001, 101.0, 105.25999999999999), 
            single::keltner_channel(
                &highs,
                &lows,
                &close,
                &crate::ConstantModelType::SimpleMovingMode,
                &crate::ConstantModelType::SimpleMovingAverage,
                &2.0
            ));
    }

    #[test]
    #[should_panic]
    fn single_keltner_channel_panic_high_length() {
        let highs = vec![101.26, 102.57, 102.32, 100.83];
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07];
        let close = vec![100.94, 101.27, 100.55, 99.01, 100.43];
        single::keltner_channel(
            &highs,
            &lows,
            &close,
            &crate::ConstantModelType::SimpleMovingMode,
            &crate::ConstantModelType::SimpleMovingAverage,
            &2.0
        );
    }

    #[test]
    #[should_panic]
    fn single_keltner_channel_panic_low_length() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83];
        let lows = vec![100.08, 98.75, 100.14, 99.07];
        let close = vec![100.94, 101.27, 100.55, 99.01, 100.43];
        single::keltner_channel(
            &highs,
            &lows,
            &close,
            &crate::ConstantModelType::SimpleMovingMode,
            &crate::ConstantModelType::SimpleMovingAverage,
            &2.0
        );
    }

    #[test]
    #[should_panic]
    fn single_keltner_channel_panic_close_length() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83];
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07];
        let close = vec![100.94, 101.27, 100.55, 100.43];
        single::keltner_channel(
            &highs,
            &lows,
            &close,
            &crate::ConstantModelType::SimpleMovingMode,
            &crate::ConstantModelType::SimpleMovingAverage,
            &2.0
        );
    }

    #[test]
    #[should_panic]
    fn single_keltner_channel_panic_empty() {
        let highs = Vec::new();
        let lows = Vec::new();
        let close = Vec::new();
        single::keltner_channel(
            &highs,
            &lows,
            &close,
            &crate::ConstantModelType::SimpleMovingMode,
            &crate::ConstantModelType::SimpleMovingAverage,
            &2.0
        );
    }

    #[test]
    fn bulk_keltner_channel() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83, 101.73, 102.01];
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        let close = vec![100.94, 101.27, 100.55, 99.01, 100.43, 101.0, 101.76];
        assert_eq!(
            vec![
                (95.99663507109007, 100.25663507109006, 104.51663507109005),
                (96.05480252764615, 100.49480252764614, 104.93480252764614),
                (97.03152290679306, 100.76352290679306, 104.49552290679306)
            ], bulk::keltner_channel(
                &highs,
                &lows,
                &close,
                &crate::ConstantModelType::ExponentialMovingAverage,
                &crate::ConstantModelType::SimpleMovingAverage,
                &2.0,
                &5_usize
            ));
    }

    #[test]
    #[should_panic]
    fn bulk_keltner_channel_panic_high_length() {
        let highs = vec![101.26, 102.57, 100.69, 100.83, 101.73, 102.01];
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        let close = vec![100.94, 101.27, 100.55, 99.01, 100.43, 101.0, 101.76];
        bulk::keltner_channel(
            &highs,
            &lows,
            &close,
            &crate::ConstantModelType::ExponentialMovingAverage,
            &crate::ConstantModelType::SimpleMovingAverage,
            &2.0,
            &5_usize
        );
    }

    #[test]
    #[should_panic]
    fn bulk_keltner_channel_panic_low_length() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83, 101.73, 102.01];
        let lows = vec![100.08, 98.75, 100.14, 99.07, 100.1, 99.96];
        let close = vec![100.94, 101.27, 100.55, 99.01, 100.43, 101.0, 101.76];
        bulk::keltner_channel(
                &highs,
                &lows,
                &close,
                &crate::ConstantModelType::ExponentialMovingAverage,
                &crate::ConstantModelType::SimpleMovingAverage,
                &2.0,
                &5_usize
        );
    }

    #[test]
    #[should_panic]
    fn bulk_keltner_channel_panic_close_length() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83, 101.73, 102.01];
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        let close = vec![100.94, 101.27, 99.01, 100.43, 101.0, 101.76];
        bulk::keltner_channel(
                &highs,
                &lows,
                &close,
                &crate::ConstantModelType::ExponentialMovingAverage,
                &crate::ConstantModelType::SimpleMovingAverage,
                &2.0,
                &5_usize
        );
    }

    #[test]
    #[should_panic]
    fn bulk_keltner_channel_panic_period() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83, 101.73, 102.01];
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        let close = vec![100.94, 101.27, 100.55, 99.01, 100.43, 101.0, 101.76];
        bulk::keltner_channel(
                &highs,
                &lows,
                &close,
                &crate::ConstantModelType::ExponentialMovingAverage,
                &crate::ConstantModelType::SimpleMovingAverage,
                &2.0,
                &50_usize
        );
    }

    #[test]
    #[should_panic]
    fn bulk_keltner_channel_panic_empty() {
        let highs = Vec::new();
        let lows = Vec::new();
        let close = Vec::new();
        bulk::keltner_channel(
                &highs,
                &lows,
                &close,
                &crate::ConstantModelType::ExponentialMovingAverage,
                &crate::ConstantModelType::SimpleMovingAverage,
                &2.0,
                &5_usize
        );
    }
}
