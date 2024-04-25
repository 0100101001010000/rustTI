//! # Momentum Indicators
//!
//! Momentum indicators show how much the price is rising or falling

/// `single` module holds functions that return a singular values
pub mod single {
    use crate::basic_indicators::single::{absolute_deviation, median, mode, standard_deviation};
    use crate::moving_average::single::{mcginley_dynamic, moving_average};
    use crate::ConstantModelType;
    use crate::DeviationModel;
    use crate::MovingAverageType;
    use crate::volatility_indicators::single::ulcer_index;
    use std::cmp::Ordering;
    /// The `relative_strenght_index` measures the speed and magnitude of price changes
    ///
    /// The period is determined based on length of `prices`. Based on the 7 day work week when the
    /// RSI was developed the default length is 14 (2 weeks) but the caller can determine their own
    /// period.
    ///
    /// # Arguments
    ///
    /// * `prices` - An `f64` slice of prices
    /// * `constant_model_type` - A variant of the `ConstantModelType` enum. The default model for
    /// the RSI as per Welles is the `ConstantModelType::SmoothedMovingAverage`
    ///
    /// # Examples
    ///
    /// ```
    /// // Short example
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0];
    ///
    /// let defaut_rsi = rust_ti::momentum_indicators::single::relative_strength_index(&prices, &rust_ti::ConstantModelType::SmoothedMovingAverage);
    /// assert_eq!(39.99999999999999, defaut_rsi);
    ///
    /// let ema_rsi = rust_ti::momentum_indicators::single::relative_strength_index(&prices, &rust_ti::ConstantModelType::ExponentialMovingAverage);
    /// assert_eq!(38.46153846153846, ema_rsi);
    ///
    /// let moving_median_rsi = rust_ti::momentum_indicators::single::relative_strength_index(&prices, &rust_ti::ConstantModelType::SimpleMovingMedian);
    /// assert_eq!(42.857142857142854, moving_median_rsi);
    /// ```
    pub fn relative_strength_index(
        prices: &[f64],
        constant_model_type: &crate::ConstantModelType,
    ) -> f64 {
        let (previous_gains, previous_loss) = previous_gains_loss(prices);
        if previous_gains.is_empty() {
            return 0.0;
        }
        if previous_loss.is_empty() {
            return 100.0;
        }

        let (previous_average_gains, previous_average_loss) = match constant_model_type {
            ConstantModelType::SimpleMovingAverage => (
                moving_average(&previous_gains, &MovingAverageType::Simple),
                moving_average(&previous_loss, &MovingAverageType::Simple),
            ),
            ConstantModelType::SmoothedMovingAverage => (
                moving_average(&previous_gains, &MovingAverageType::Smoothed),
                moving_average(&previous_loss, &MovingAverageType::Smoothed),
            ),
            ConstantModelType::ExponentialMovingAverage => (
                moving_average(&previous_gains, &MovingAverageType::Exponential),
                moving_average(&previous_loss, &MovingAverageType::Exponential),
            ),
            ConstantModelType::PersonalisedMovingAverage(alpha_nominator, alpha_denominator) => (
                moving_average(
                    &previous_gains,
                    &MovingAverageType::Personalised(alpha_nominator, alpha_denominator),
                ),
                moving_average(
                    &previous_loss,
                    &MovingAverageType::Personalised(alpha_nominator, alpha_denominator),
                ),
            ),
            ConstantModelType::SimpleMovingMedian => {
                (median(&previous_gains), median(&previous_loss))
            }
            ConstantModelType::SimpleMovingMode => (mode(&previous_gains), mode(&previous_loss)),
            _ => panic!("Unsupported ConstantModelType"),
        };
        return rsi(&previous_average_gains, &previous_average_loss);
    }

    /// The `mcginley_dynamic_rsi` is a variation of the `relative_strength_index` that accepts
    /// previous dynamics for the gains and loss calculations, but also returns the new ones so
    /// that they can be used going forward
    ///
    /// # Arguments
    ///  
    /// * `prices` - An `f64` slice of prices
    /// * `previous_gain_mcginley_dynamic` - The previous McGinley dynamic used for the gains
    /// calculation. Use 0.0 if it hasn't yet been calculated.
    /// * `previous_loss_mcginley_dynamic` - The previous McGinley dynamic used for the loss
    /// caclulation. Use 0.0 if it hasn't yet been calculated.
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0];
    /// let previous_gain_dynamic = 0.0;
    /// let previous_loss_dynamic = 0.0;
    /// let mcginley_rsi = rust_ti::momentum_indicators::single::mcginley_dynamic_rsi(&prices,
    /// &previous_gain_dynamic, &previous_loss_dynamic);
    /// assert_eq!((33.33333333333333, 1.0, 2.0), mcginley_rsi);
    ///
    /// let prices = vec![102.0, 103.0, 101.0, 99.0, 97.0];
    /// let previous_gain_dynamic = mcginley_rsi.1;
    /// let previous_loss_dynamic = mcginley_rsi.2;
    /// let mcginley_rsi = rust_ti::momentum_indicators::single::mcginley_dynamic_rsi(&prices,
    /// &previous_gain_dynamic, &previous_loss_dynamic);
    /// assert_eq!((33.33333333333333, 1.0, 2.0), mcginley_rsi);
    /// ```
    pub fn mcginley_dynamic_rsi(
        prices: &[f64],
        previous_gain_mcginley_dynamic: &f64,
        previous_loss_mcginley_dynamic: &f64,
    ) -> (f64, f64, f64) {
        let (previous_gains, previous_loss) = previous_gains_loss(prices);
        if previous_gains.is_empty() {
            return (
                0.0,
                *previous_gain_mcginley_dynamic,
                mcginley_dynamic(
                    &previous_loss.last().unwrap(),
                    previous_loss_mcginley_dynamic,
                    &previous_loss.len(),
                ),
            );
        };
        if previous_loss.is_empty() {
            return (
                100.0,
                mcginley_dynamic(
                    &previous_gains.last().unwrap(),
                    previous_gain_mcginley_dynamic,
                    &previous_gains.len(),
                ),
                *previous_loss_mcginley_dynamic,
            );
        }
        let previous_gain_dynamic = mcginley_dynamic(
            &previous_gains.last().unwrap(),
            previous_gain_mcginley_dynamic,
            &previous_gains.len(),
        );
        let previous_loss_dynamic = mcginley_dynamic(
            &previous_loss.last().unwrap(),
            previous_loss_mcginley_dynamic,
            &previous_loss.len(),
        );
        return (
            rsi(&previous_gain_dynamic, &previous_loss_dynamic),
            previous_gain_dynamic,
            previous_loss_dynamic,
        );
    }

    /// The `stochastic_oscillator` is a momentum indicator that is calculated by using support and
    /// resistance levels
    ///
    /// When developed by George Lane the stochastic oscillator looked at a period of 14 days as
    /// the work week had 7 days back then, however this implementation allows for prices of any
    /// length to be used
    ///
    /// # Arguments
    ///
    /// * `prices` - An `f64` slice of prices
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0];
    /// let stochastic_oscillator = rust_ti::momentum_indicators::single::stochastic_oscillator(&prices);
    /// assert_eq!(0.0, stochastic_oscillator);
    /// ```
    pub fn stochastic_oscillator(prices: &[f64]) -> f64 {
        if prices.is_empty() {
            panic!("Prices is empty");
        };
        let mut ordered_prices = prices
            .iter()
            .filter_map(|f| if f.is_nan() { None } else { Some(*f) })
            .collect::<Vec<f64>>();
        ordered_prices.sort_by(cmp_f64);
        let min = ordered_prices[0];
        let max = ordered_prices.last().unwrap();
        return 100.0 * ((prices.last().unwrap() - min) / (max - min));
    }

    /// The `slow_stochastic` is a momentum indicator that takes the moving average of passed in
    /// stochastic oscillators
    ///
    /// The standard length of stochastic oscillators is 3, and the constant model is a simple moving average
    ///
    /// # Arguments
    ///
    /// * `stochastics` - An `f64` slice of stochastics
    /// * `constant_model_type` - A variant of `ConstantModelType`
    ///
    /// # Examples
    ///
    /// ```
    /// let stochstic_oscillators = [0.0, 50.0, 100.0];
    ///
    /// let simple_ma_slow_stochastic = rust_ti::momentum_indicators::single::slow_stochastic(&stochstic_oscillators, &rust_ti::ConstantModelType::SimpleMovingAverage);
    /// assert_eq!(50.0, simple_ma_slow_stochastic);
    ///
    /// let sma_slow_stochastic =
    /// rust_ti::momentum_indicators::single::slow_stochastic(&stochstic_oscillators,
    /// &rust_ti::ConstantModelType::SmoothedMovingAverage);
    /// assert_eq!(63.15789473684211, sma_slow_stochastic);
    ///
    /// let median_slow_stochastic = rust_ti::momentum_indicators::single::slow_stochastic(&stochstic_oscillators, &rust_ti::ConstantModelType::SimpleMovingMedian);
    /// assert_eq!(50.0, median_slow_stochastic);
    /// ```
    pub fn slow_stochastic(
        stochastics: &[f64],
        constant_model_type: &crate::ConstantModelType,
    ) -> f64 {
        if stochastics.is_empty() {
            panic!("stochastics cannot be empty");
        };

        let slow_stochastic = match constant_model_type {
            ConstantModelType::SimpleMovingAverage => {
                moving_average(&stochastics, &MovingAverageType::Simple)
            }
            ConstantModelType::SmoothedMovingAverage => {
                moving_average(&stochastics, &MovingAverageType::Smoothed)
            }
            ConstantModelType::ExponentialMovingAverage => {
                moving_average(&stochastics, &MovingAverageType::Exponential)
            }
            ConstantModelType::PersonalisedMovingAverage(alpha_nominator, alpha_denominator) => {
                moving_average(
                    &stochastics,
                    &MovingAverageType::Personalised(alpha_nominator, alpha_denominator),
                )
            }
            ConstantModelType::SimpleMovingMedian => median(&stochastics),
            ConstantModelType::SimpleMovingMode => mode(&stochastics),
            _ => panic!("Unsupported ConstantModelType"),
        };
        return slow_stochastic;
    }

    /// The `slowest_stochastic` is a momentum indicator that takes the moving average of passed in
    /// slow stochastic oscillators
    ///
    /// The standard length of slow stochastics is 3, and the constant model is a simple moving average
    ///
    /// # Arguments
    ///
    /// * `slow_stochastics` - An `f64` slice of slow stochastics
    /// * `constant_model_type` - A variant of `ConstantModelType`
    ///
    /// # Examples
    ///
    /// ```
    /// let slow_stochstic = [30.0, 20.0, 10.0];
    ///
    /// let simple_ma_slowest_stochastic = rust_ti::momentum_indicators::single::slowest_stochastic(&slow_stochstic, &rust_ti::ConstantModelType::SimpleMovingAverage);
    /// assert_eq!(20.0, simple_ma_slowest_stochastic);
    ///
    /// let sma_slowest_stochastic =
    /// rust_ti::momentum_indicators::single::slowest_stochastic(&slow_stochstic,
    /// &rust_ti::ConstantModelType::SmoothedMovingAverage);
    /// assert_eq!(17.368421052631582, sma_slowest_stochastic);
    ///
    /// let median_slowest_stochastic = rust_ti::momentum_indicators::single::slowest_stochastic(&slow_stochstic, &rust_ti::ConstantModelType::SimpleMovingMedian);
    /// assert_eq!(20.0, median_slowest_stochastic);
    /// ```
    pub fn slowest_stochastic(
        slow_stochastics: &[f64],
        constant_model_type: &crate::ConstantModelType,
    ) -> f64 {
        if slow_stochastics.is_empty() {
            panic!("stochastics cannot be empty");
        };

        let slowest_stochastic = match constant_model_type {
            ConstantModelType::SimpleMovingAverage => {
                moving_average(&slow_stochastics, &MovingAverageType::Simple)
            }
            ConstantModelType::SmoothedMovingAverage => {
                moving_average(&slow_stochastics, &MovingAverageType::Smoothed)
            }
            ConstantModelType::ExponentialMovingAverage => {
                moving_average(&slow_stochastics, &MovingAverageType::Exponential)
            }
            ConstantModelType::PersonalisedMovingAverage(alpha_nominator, alpha_denominator) => {
                moving_average(
                    &slow_stochastics,
                    &MovingAverageType::Personalised(alpha_nominator, alpha_denominator),
                )
            }
            ConstantModelType::SimpleMovingMedian => median(&slow_stochastics),
            ConstantModelType::SimpleMovingMode => mode(&slow_stochastics),
            _ => panic!("Unsupported ConstantModelType"),
        };
        return slowest_stochastic;
    }

    /// `williams_percent_r` is a momentum that tracks overbought and oversold levels.
    ///
    /// The standard period used is 14 days. The high would be the maximum price over the past 14 days, and the low the minimum price over the past 14 days.
    ///
    /// # Arguments
    ///
    /// * `high` - High price for the observed period
    /// * `low` - Low price for the observed period
    /// * `close` - Close price for the observed period
    ///
    /// # Examples
    ///
    /// ```
    /// let high = 200.0;
    /// let low = 175.0;
    /// let close = 192.0;
    /// let williams_percent_r = rust_ti::momentum_indicators::single::williams_percent_r(&high, &low,
    /// &close);
    /// assert_eq!(-32.0, williams_percent_r);
    /// ```
    pub fn williams_percent_r(high: &f64, low: &f64, close: &f64) -> f64 {
        return -100.0_f64 * ((high - close) / (high - low));
    }

    /// The `money_flow_index` is a momentum indicator that shows the volume of money (a.k.a money
    /// flow) going in and out of the asset.
    ///
    /// The standard money flow index uses the typical price (high+low+close/3) as the observed
    /// price and a period of 14 days.
    ///
    /// # Arguments
    ///
    /// * `prices` - An `f64` slice of prices
    /// * `volume` - Volume of transactions
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0];
    /// let volume = vec![1000.0, 1500.0, 1200.0, 900.0, 1300.0];
    /// let money_flow_index = rust_ti::momentum_indicators::single::money_flow_index(&prices,
    /// &volume);
    /// assert_eq!(56.771463119709786, money_flow_index);
    /// ```
    pub fn money_flow_index(prices: &[f64], volume: &[f64]) -> f64 {
        if prices.is_empty() {
            panic!("Prices cannot be empty");
        }
        let length = prices.len();
        if length != volume.len() {
            panic!(
                "Length of prices ({}) needs to length of volume ({})",
                length,
                volume.len()
            );
        };

        let mut raw_money_flow = Vec::new();
        for i in 0..length {
            raw_money_flow.push(prices[i] * volume[i]);
        }

        let mut positive_money_flow = 0.0;
        let mut negative_money_flow = 0.0;
        for (i, value) in raw_money_flow.iter().enumerate() {
            if i == 0 {
                continue;
            };
            if value > &raw_money_flow[i - 1] {
                positive_money_flow = positive_money_flow + value;
            } else if value < &raw_money_flow[i - 1] {
                negative_money_flow = negative_money_flow + value;
            };
        }

        if negative_money_flow == 0.0 {
            return 100.0;
        };
        return 100.0 - (100.0 / (1.0 + (positive_money_flow / negative_money_flow)));
    }

    // TODO: Chaikin Oscillator once accumulation distribution is done

    /// The `rate_of_change` is a momentum indicator that shows how quickly the price of an asset
    /// is changing.
    ///
    /// The standard is to use the closing price.
    ///
    /// # Arguments
    ///
    /// * `current_price` - Price at t
    /// * `previous_price` - Price at t-n
    ///
    /// # Examples
    ///
    /// ```
    /// let current_price = 120.0;
    /// let previous_price = 100.0;
    /// let rate_of_change = rust_ti::momentum_indicators::single::rate_of_change(&current_price,
    /// &previous_price);
    /// assert_eq!(20.0, rate_of_change);
    ///
    /// let current_price = 100.0;
    /// let previous_price = 120.0;
    /// let rate_of_change = rust_ti::momentum_indicators::single::rate_of_change(&current_price,
    /// &previous_price);
    /// assert_eq!(-16.666666666666664, rate_of_change);
    /// ```
    pub fn rate_of_change(current_price: &f64, previous_price: &f64) -> f64 {
        return ((current_price - previous_price) / previous_price) * 100.0;
    }

    /// The `on_balance_volume` is a momentum indicator that uses volume
    ///
    /// The standard price to use it the closing price. If there is no previous on balance volume
    /// pass in 0.
    ///
    /// # Arguments
    ///
    /// * `current_price` - Price a t
    /// * `previous_price` - Price at t-n
    /// * `current_volume` - Volume at t
    /// * `previous_on_balance_volume` - Previous on balance volume. Use 0 if no previous OBV.
    ///
    /// # Examples
    ///
    /// ```
    /// let current_price = 120.0;
    /// let previous_price = 100.0;
    /// let current_volume = 1500;
    /// let on_balance_volume =
    /// rust_ti::momentum_indicators::single::on_balance_volume(&current_price, &previous_price,
    /// &current_volume, &0);
    /// assert_eq!(1500 ,on_balance_volume);
    ///
    /// let current_price = 100.0;
    /// let previous_price = 120.0;
    /// let current_volume = 1000;
    /// let on_balance_volume =
    /// rust_ti::momentum_indicators::single::on_balance_volume(&current_price, &previous_price,
    /// &current_volume, &1500);
    /// assert_eq!(500, on_balance_volume);
    /// ```
    pub fn on_balance_volume(
        current_price: &f64,
        previous_price: &f64,
        current_volume: &i64,
        previous_on_balance_volume: &i64,
    ) -> i64 {
        let mut volume = 0;
        if current_price > previous_price {
            volume = volume + current_volume;
        } else if current_price < previous_price {
            volume = volume - current_volume;
        };
        return previous_on_balance_volume + volume;
    }

    /// The `commodity_channel_index` is a momentum indicator flagging overbought and oversold
    /// conditions.
    ///
    /// The standard commodity channel index uses the typical price, a simple moving average model,
    /// and a mean absolute deviation model. The `constant_multiplier` is a scale factor used to
    /// provide more readable numbers. It is recommended to use 0.015 as the default as changing it
    /// will impact how the numbers are distributed and the standard -100/100 flags may no longer
    /// apply.
    ///
    /// # Arguments
    ///
    /// `prices` - An `f64` slice of prices
    /// `constant_model_type` - A variant of `ConstantModelType`
    /// `deviation_model` - A variant of `DeviationModel`
    /// `constant_multiplier` - Scale factor. Normally 0.015
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0];
    /// let constant_multiplier = 0.015;
    ///
    /// let default_cci = rust_ti::momentum_indicators::single::commodity_channel_index(&prices, &rust_ti::ConstantModelType::SimpleMovingAverage, &rust_ti::DeviationModel::MeanAbsoluteDeviation, &constant_multiplier);
    /// assert_eq!(-111.11111111111111, default_cci);
    ///
    /// let ema_sd_cci = rust_ti::momentum_indicators::single::commodity_channel_index(&prices,
    /// &rust_ti::ConstantModelType::ExponentialMovingAverage,
    /// &rust_ti::DeviationModel::StandardDeviation, &constant_multiplier);
    /// assert_eq!(-75.96091804215764, ema_sd_cci);
    ///
    /// let median_mad_cci = rust_ti::momentum_indicators::single::commodity_channel_index(&prices,
    /// &rust_ti::ConstantModelType::SimpleMovingMedian,
    /// &rust_ti::DeviationModel::MedianAbsoluteDeviation, &constant_multiplier);  
    /// assert_eq!(-111.11111111111111, median_mad_cci);
    /// ```
    pub fn commodity_channel_index(
        prices: &[f64],
        constant_model_type: &crate::ConstantModelType,
        deviation_model: &crate::DeviationModel,
        constant_multiplier: &f64,
    ) -> f64 {
        if prices.is_empty() {
            panic!("Prices cannot be empty");
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
            DeviationModel::UlcerIndex => {
                ulcer_index(&prices)
            }
            _ => panic!("Unsupported DeviationModel"),
        };
        return (prices.last().unwrap() - moving_constant) / (constant_multiplier * deviation);
    }

    /// The `mcginley_dynamic_commodity_channel_index` is a momentum indicator flagging overbought and oversold
    /// conditions.
    ///
    /// The McGinley dynamic commodity channel index uses the McGinley dynamic rather than a moving average model, and returns the CCI as well as the McGinley dynamic.
    /// The caller still needs to determine the absolute deviation model.
    ///
    /// # Arguments
    ///
    /// `prices` - An `f64` slice of prices
    /// `previous_mcginley_dynamic` - The previous value of the McGinley dynamic. 0.0 if no
    /// previous.
    /// `deviation_model` - A variant of `DeviationModel`
    /// `constant_multiplier` - Scale factor. Normally 0.015
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0];
    /// let constant_multiplier = 0.015;
    ///
    /// let mcginley_cci = rust_ti::momentum_indicators::single::mcginley_dynamic_commodity_channel_index(
    /// &prices, &0.0, &rust_ti::DeviationModel::MeanAbsoluteDeviation, &constant_multiplier);
    /// assert_eq!((0.0, 99.0), mcginley_cci);
    ///
    /// let prices = vec![102.0, 103.0, 101.0, 99.0, 102.0];
    /// let mcginley_cci = rust_ti::momentum_indicators::single::mcginley_dynamic_commodity_channel_index(
    /// &prices, &mcginley_cci.1, &rust_ti::DeviationModel::MeanAbsoluteDeviation, &constant_multiplier);
    /// assert_eq!((146.8770632107927, 99.53246533805869), mcginley_cci);
    /// ```
    pub fn mcginley_dynamic_commodity_channel_index(
        prices: &[f64],
        previous_mcginley_dynamic: &f64,
        deviation_model: &crate::DeviationModel,
        constant_multiplier: &f64,
    ) -> (f64, f64) {
        if prices.is_empty() {
            panic!("Prices cannot be empty");
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
            DeviationModel::UlcerIndex => {
                ulcer_index(&prices)
            }
            _ => panic!("Unsupported DeviationModel"),
        };
        return (
            (last_price - mcginley_dynamic) / (constant_multiplier * deviation),
            mcginley_dynamic,
        );
    }

    /// The `macd_line` is a momentum indicator that also shows price
    /// strength, direction, and general trend.
    ///
    /// The indicator was developer back when there was a 6 day trading week so the standard short
    /// term to use is 12, and 26 for the long term. However the caller can determine the period.
    /// Both tyically use an exponential moving average model, but this is also determined by the
    /// caller.
    ///
    /// The long period is determined by the length of the `prices` slice.
    ///
    /// This function *only* calculates the MACD line not the signal line, `signal_line` should be
    /// called for the signal line.
    ///
    /// # Arguments
    ///
    /// * `prices` - An `f64` slice of prices
    /// * `short_period` - The length of the short period
    /// * `short_period_model` - A variant of `ConstantModelType`
    /// * `long_period_model` - A variant of `ConstantModelType`
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0];
    ///
    /// let macd = rust_ti::momentum_indicators::single::macd_line(&prices, &3_usize,
    /// &rust_ti::ConstantModelType::ExponentialMovingAverage,
    /// &rust_ti::ConstantModelType::ExponentialMovingAverage);
    /// assert_eq!(-0.46851726472581845, macd);
    ///
    /// let macd = rust_ti::momentum_indicators::single::macd_line(&prices, &3_usize,
    /// &rust_ti::ConstantModelType::SimpleMovingAverage,
    /// &rust_ti::ConstantModelType::SimpleMovingMedian);
    /// assert_eq!(0.0, macd);
    /// ```
    pub fn macd_line(
        prices: &[f64],
        short_period: &usize,
        short_period_model: &crate::ConstantModelType,
        long_period_model: &crate::ConstantModelType,
    ) -> f64 {
        if prices.is_empty() {
            panic!("Prices cannot be empty");
        };
        let length = prices.len();
        if short_period >= &length {
            panic!(
                "Short period ({}) cannot be greater or equal to long period ({})",
                short_period, length
            );
        };

        let short_period_index = short_period - 1;
        let short_period_average = match short_period_model {
            ConstantModelType::SimpleMovingAverage => {
                moving_average(&prices[short_period_index..], &MovingAverageType::Simple)
            }
            ConstantModelType::SmoothedMovingAverage => {
                moving_average(&prices[short_period_index..], &MovingAverageType::Smoothed)
            }
            ConstantModelType::ExponentialMovingAverage => moving_average(
                &prices[short_period_index..],
                &MovingAverageType::Exponential,
            ),
            ConstantModelType::PersonalisedMovingAverage(alpha_nominator, alpha_denominator) => {
                moving_average(
                    &prices[short_period_index..],
                    &MovingAverageType::Personalised(alpha_nominator, alpha_denominator),
                )
            }
            ConstantModelType::SimpleMovingMedian => median(&prices[short_period_index..]),
            ConstantModelType::SimpleMovingMode => mode(&prices[short_period_index..]),
            _ => panic!("Unsupported ConstantModelType"),
        };

        let long_period_average = match long_period_model {
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
        return short_period_average - long_period_average;
    }

    /// The `signal_line` is used with the `macd_line` to produce the moving average convergence
    /// divergence.
    ///
    /// The standard period for the signal line is 9, and it normally uses an exponential moving
    /// average model.
    ///
    /// # Arguments
    ///
    /// * `macds` - An `f64` slice of MACDs
    /// * constant_model_type` - A variant of `ConstantModelType`
    ///
    /// # Examples
    ///
    /// ```
    /// let macds = vec![-0.0606702775897219, -0.0224170616113781, 0.0057887610020515];
    ///
    /// let ema_signal_line = rust_ti::momentum_indicators::single::signal_line(&macds,
    /// &rust_ti::ConstantModelType::ExponentialMovingAverage);
    /// assert_eq!(-0.011764193829181728, ema_signal_line);
    ///
    /// let median_signal_line = rust_ti::momentum_indicators::single::signal_line(&macds,
    /// &rust_ti::ConstantModelType::SimpleMovingMedian);
    /// assert_eq!(-0.0224170616113781, median_signal_line);
    /// ```
    pub fn signal_line(macds: &[f64], constant_model_type: &crate::ConstantModelType) -> f64 {
        if macds.is_empty() {
            panic!("macds cannot be empty");
        };

        match constant_model_type {
            ConstantModelType::SimpleMovingAverage => {
                return moving_average(&macds, &MovingAverageType::Simple)
            }
            ConstantModelType::SmoothedMovingAverage => {
                return moving_average(&macds, &MovingAverageType::Smoothed)
            }
            ConstantModelType::ExponentialMovingAverage => {
                return moving_average(&macds, &MovingAverageType::Exponential)
            }
            ConstantModelType::PersonalisedMovingAverage(alpha_nominator, alpha_denominator) => {
                return moving_average(
                    &macds,
                    &MovingAverageType::Personalised(alpha_nominator, alpha_denominator),
                )
            }
            ConstantModelType::SimpleMovingMedian => return median(&macds),
            ConstantModelType::SimpleMovingMode => return mode(&macds),
            _ => panic!("Unsupported ConstantModelType"),
        }
    }

    /// The `mcginley_dynamic_macd_line` is an alternative to `macd_line` that uses and returns the
    /// McGinley Dynamic as well as the MACD.
    ///
    /// The function returns a tuple with the MACD value first, short period McGinley dynamic second, and the long period third.
    ///
    /// The is no McGinley dynamic just for the signal line as the singal line merely take a moving
    /// average of the MACDs, for a McGinley dynamic version just call
    /// `mcginley_dynamic` from `moving_averages.rs`
    ///
    /// # Arguments
    ///
    /// * `prices` - An `f64` slice of prices
    /// * `short_period` - The length of the short period
    /// * `previous_short_mcginley` - Previous McGinley dynamic for the short model. If no
    /// previous use 0.0.
    /// * `previous_long_mcginley` - Previous McGinley dynamic for the long model. If no
    /// previous use 0.0.
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0];
    /// let short_period: usize = 3;
    ///
    /// let mcginley_dynamic_macd = rust_ti::momentum_indicators::single::mcginley_dynamic_macd_line(&prices, &short_period,
    /// &0.0, &0.0);
    /// assert_eq!((0.0, 99.0, 99.0), mcginley_dynamic_macd);
    ///
    /// let prices = vec![102.0, 103.0, 101.0, 99.0, 102.0];
    /// let mcginley_dynamic_macd = rust_ti::momentum_indicators::single::mcginley_dynamic_macd_line(&prices, &short_period,
    /// &mcginley_dynamic_macd.1, &mcginley_dynamic_macd.2);
    /// assert_eq!((0.35497689203913296, 99.88744223009782, 99.53246533805869), mcginley_dynamic_macd);
    /// ```
    pub fn mcginley_dynamic_macd_line(
        prices: &[f64],
        short_period: &usize,
        previous_short_mcginley: &f64,
        previous_long_mcginley: &f64,
    ) -> (f64, f64, f64) {
        if prices.is_empty() {
            panic!("Prices cannot be empty");
        };

        if short_period >= &prices.len() {
            panic!(
                "Short period ({}) cannot be longer or equal to length of prices ({})",
                short_period,
                prices.len()
            );
        };

        let latest_price = prices.last().unwrap();
        if previous_short_mcginley == &0.0 && previous_long_mcginley == &0.0 {
            return (0.0, *latest_price, *latest_price);
        };

        let long_mcginley = mcginley_dynamic(&latest_price, previous_long_mcginley, &prices.len());
        let short_mcginley = mcginley_dynamic(&latest_price, previous_short_mcginley, short_period);
        let macd = short_mcginley - long_mcginley;
        return (macd, short_mcginley, long_mcginley);
    }

    fn previous_gains_loss(prices: &[f64]) -> (Vec<f64>, Vec<f64>) {
        if prices.is_empty() {
            panic!("Prices is empty");
        };
        let mut previous_gains = Vec::new();
        let mut previous_loss = Vec::new();
        for (i, value) in prices.iter().enumerate() {
            // TODO: must be a better way to do this
            if i == 0 {
                continue;
            };
            if value > &prices[i - 1] {
                previous_gains.push(value - prices[i - 1]);
            } else if value < &prices[i - 1] {
                previous_loss.push(prices[i - 1] - value);
            };
        }
        return (previous_gains, previous_loss);
    }

    fn rsi(previous_average_gains: &f64, previous_average_loss: &f64) -> f64 {
        if previous_average_loss == &0.0_f64 {
            return 0.0;
        };
        return 100.0_f64
            - (100.0_f64 / (1.0_f64 + (previous_average_gains / previous_average_loss)));
    }

    fn cmp_f64(a: &f64, b: &f64) -> Ordering {
        if a < b {
            return Ordering::Less;
        } else if a > b {
            return Ordering::Greater;
        }
        return Ordering::Equal;
    }
}

/// `bulk` module holds functions that return multiple valus for `momentum_indicators`
pub mod bulk {
    use crate::momentum_indicators::single;
    /// The `relative_strenght_index` measures the speed and magnitude of price changes
    ///
    /// Based on the 7 day work week when the RSI was developed the default length is 14 (2 weeks) but the caller can determine their own
    /// period.
    ///
    /// # Arguments
    ///
    /// * `prices` - An `f64` slice of prices
    /// * `constant_model_type` - A variant of the `ConstantModelType` enum. The default model for
    /// the RSI as per Welles is the `ConstantModelType::SmoothedMovingAverage`
    /// * `period` - A `usize` period over which to calculate the RSI
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0];
    /// let period: usize = 3;
    /// let defaut_rsi = rust_ti::momentum_indicators::bulk::relative_strength_index(&prices, &rust_ti::ConstantModelType::SmoothedMovingAverage, &period);
    /// assert_eq!(vec![100.0, 33.33333333333333, 0.0], defaut_rsi);
    ///
    /// let ema_rsi = rust_ti::momentum_indicators::bulk::relative_strength_index(&prices, &rust_ti::ConstantModelType::ExponentialMovingAverage, &period);
    /// assert_eq!(vec![100.0, 33.33333333333333, 0.0], ema_rsi);
    ///
    /// let moving_median_rsi = rust_ti::momentum_indicators::bulk::relative_strength_index(&prices, &rust_ti::ConstantModelType::SimpleMovingMedian, &period);
    /// assert_eq!(vec![100.0, 33.33333333333333, 0.0], moving_median_rsi);
    ///
    /// // Long example
    /// let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
    /// let period: usize = 5;
    ///
    /// let personalised_rsi = rust_ti::momentum_indicators::bulk::relative_strength_index(&prices, &rust_ti::ConstantModelType::PersonalisedMovingAverage(&5.0, &4.0), &period);
    /// assert_eq!(vec![34.51776649746324, 12.837837837836929, 34.51776649746182, 61.26126126126377], personalised_rsi);
    /// ```
    pub fn relative_strength_index(
        prices: &[f64],
        constant_model_type: &crate::ConstantModelType,
        period: &usize,
    ) -> Vec<f64> {
        let length = prices.len();
        if period > &length {
            panic!(
                "Period ({}) is longer than length of prices ({})",
                period, length
            );
        };

        let mut rsis = Vec::new();
        let loop_max = length - period + 1;
        for i in 0..loop_max {
            rsis.push(single::relative_strength_index(
                &prices[i..i + period],
                constant_model_type,
            ));
        }
        return rsis;
    }
    /// The `mcginley_dynamic_rsi` is a variation of the `relative_strength_index` that accepts
    /// previous dynamics for the gains and loss calculations, but also returns the new ones so
    /// that they can be used going forward
    ///
    /// # Arguments
    ///  
    /// * `prices` - An `f64` slice of prices
    /// * `previous_gain_mcginley_dynamic` - The previous McGinley dynamic used for the gains
    /// calculation. Use 0.0 if it hasn't yet been calculated.
    /// * `previous_loss_mcginley_dynamic` - The previous McGinley dynamic used for the loss
    /// caclulation. Use 0.0 if it hasn't yet been calculated.
    /// * `period` - Period over which to calculate the McGinley dynamic RSI
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 100.0, 97.0, 102.0, 103.0];
    /// let period: usize = 5;
    /// let previous_gain_dynamic = 0.0;
    /// let previous_loss_dynamic = 0.0;
    /// let mcginley_rsi = rust_ti::momentum_indicators::bulk::mcginley_dynamic_rsi(&prices,
    /// &previous_gain_dynamic, &previous_loss_dynamic, &period);
    /// assert_eq!(vec![(50.0, 1.0, 1.0), (49.795081967213115, 1.0, 1.008230452674897),
    /// (49.745434479572864, 1.0064, 1.0167002312649644), (49.344186087882605, 1.003117290207188, 1.0297813544692562)], mcginley_rsi);
    /// ```
    pub fn mcginley_dynamic_rsi(
        prices: &[f64],
        previous_gain_mcginley_dynamic: &f64,
        previous_loss_mcginley_dynamic: &f64,
        period: &usize,
    ) -> Vec<(f64, f64, f64)> {
        let length = prices.len();
        if &length < period {
            panic!(
                "Period ({}) is longer than length ({}) of prices",
                period, length
            );
        };
        let mut rsis = vec![single::mcginley_dynamic_rsi(
            &prices[..*period],
            previous_gain_mcginley_dynamic,
            previous_loss_mcginley_dynamic,
        )];
        let loop_max = length - period + 1;
        for i in 1..loop_max {
            let previous_rsi = rsis[i - 1];
            rsis.push(single::mcginley_dynamic_rsi(
                &prices[i..i + period],
                &previous_rsi.1,
                &previous_rsi.2,
            ));
        }
        return rsis;
    }

    /// The `stochastic_oscillator` is a momentum indicator that is calculated by using support and
    /// resistance levels
    ///
    /// When developed by George Lane the stochastic oscillator looked at a period of 14 days as
    /// the work week had 7 days back then, however this implementation allows for prices of any
    /// length to be used
    ///
    /// # Arguments
    ///
    /// * `prices` - An `f64` slice of prices
    /// * `period` - Period over which to calculate the stochastic oscillator
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0];
    /// let period: usize = 3;
    /// let stochastic_oscillator = rust_ti::momentum_indicators::bulk::stochastic_oscillator(&prices, &period);
    /// assert_eq!(vec![100.0, 0.0, 0.0], stochastic_oscillator);
    /// ```
    pub fn stochastic_oscillator(prices: &[f64], period: &usize) -> Vec<f64> {
        let length = prices.len();
        if period > &length {
            panic!(
                "Period ({}) cannot be greater than length ({}) of prices",
                period, length
            );
        };

        let mut so = Vec::new();
        let loop_max = length - period + 1;
        for i in 0..loop_max {
            so.push(single::stochastic_oscillator(&prices[i..i + period]));
        }
        return so;
    }

    /// The `slow_stochastic` is a momentum indicator that takes the moving average of passed in
    /// stochastic oscillators
    ///
    /// The standard length of prices is 3, and the constant model is a simple moving average
    ///
    /// # Arguments
    ///
    /// * `stochastics` - An `f64` slice of prices
    /// * `constant_model_type` - A variant of `ConstantModelType`
    /// * `period` - Period over which to calculate the stochastic oscillator
    ///
    /// # Examples
    ///
    /// ```
    /// let stochstic_oscillators = [0.0, 25.0, 50.0, 33.0, 73.0];
    /// let period: usize = 3;
    ///
    /// let simple_ma_slow_stochastic = rust_ti::momentum_indicators::bulk::slow_stochastic(&stochstic_oscillators, &rust_ti::ConstantModelType::SimpleMovingAverage, &period);
    /// assert_eq!(vec![25.0, 36.0, 52.0], simple_ma_slow_stochastic);
    ///
    /// let sma_slow_stochastic =
    /// rust_ti::momentum_indicators::bulk::slow_stochastic(&stochstic_oscillators,
    /// &rust_ti::ConstantModelType::SmoothedMovingAverage, &period);
    /// assert_eq!(vec![31.578947368421055, 36.684210526315795, 55.526315789473685], sma_slow_stochastic);
    ///
    /// let median_slow_stochastic = rust_ti::momentum_indicators::bulk::slow_stochastic(&stochstic_oscillators, &rust_ti::ConstantModelType::SimpleMovingMedian, &period);
    /// assert_eq!(vec![25.0, 33.0, 50.0], median_slow_stochastic);
    /// ```
    pub fn slow_stochastic(
        stochastics: &[f64],
        constant_model_type: &crate::ConstantModelType,
        period: &usize,
    ) -> Vec<f64> {
        let length = stochastics.len();
        if period > &length {
            panic!(
                "Period ({}) cannot be greater than length ({}) of stochastics",
                period, length
            );
        };
        let mut sso = Vec::new();
        let loop_max = length - period + 1;
        for i in 0..loop_max {
            sso.push(single::slow_stochastic(
                &stochastics[i..i + period],
                constant_model_type,
            ));
        }
        return sso;
    }

    /// The `slowest_stochastic` is a momentum indicator that takes the moving average of passed in
    /// slow stochastics
    ///
    /// The standard length of prices is 3, and the constant model is a simple moving average
    ///
    /// # Arguments
    ///
    /// * `slow_stochastics` - An `f64` slice of prices
    /// * `constant_model_type` - A variant of `ConstantModelType`
    /// * `period` - Period over which to calculate the stochastic oscillator
    ///
    /// # Examples
    ///
    /// ```
    /// let slow_stochstics = [75.0, 60.0, 73.0, 58.0];
    /// let period: usize = 3;
    ///
    /// let ma_slowest_stochastic = rust_ti::momentum_indicators::bulk::slowest_stochastic(&slow_stochstics, &rust_ti::ConstantModelType::SimpleMovingAverage, &period);
    /// assert_eq!(vec![69.33333333333333, 63.666666666666664], ma_slowest_stochastic);
    ///
    /// let sma_slowest_stochastic =
    /// rust_ti::momentum_indicators::bulk::slow_stochastic(&slow_stochstics,
    /// &rust_ti::ConstantModelType::SmoothedMovingAverage, &period);
    /// assert_eq!(vec![69.31578947368422, 63.15789473684211], sma_slowest_stochastic);
    ///
    /// let median_slowest_stochastic = rust_ti::momentum_indicators::bulk::slow_stochastic(&slow_stochstics, &rust_ti::ConstantModelType::SimpleMovingMedian, &period);
    /// assert_eq!(vec![73.0, 60.0], median_slowest_stochastic);
    /// ```
    pub fn slowest_stochastic(
        slow_stochastics: &[f64],
        constant_model_type: &crate::ConstantModelType,
        period: &usize,
    ) -> Vec<f64> {
        let length = slow_stochastics.len();
        if period > &length {
            panic!(
                "Period ({}) cannot be greater than length ({}) of stochastics",
                period, length
            );
        };
        let mut sso = Vec::new();
        let loop_max = length - period + 1;
        for i in 0..loop_max {
            sso.push(single::slowest_stochastic(
                &slow_stochastics[i..i + period],
                constant_model_type,
            ));
        }
        return sso;
    }

    /// The `williams_percent_r` is a momentum that tracks overbought and oversold levels.
    ///
    /// The standard period used is 14 days. The high would be the maximum price over the past 14 days, and the low the minimum price over the past 14 days.
    ///
    /// The function doesn't have any logic to determine period highs and lows, it is up to the
    /// caller to ensure that the highs, lows, close, are for the same period.
    ///
    /// # Arguments
    ///
    /// * `high` - High price for the observed period
    /// * `low` - Low price for the observed period
    /// * `close` - Close price for the observed period
    ///
    /// # Examples
    ///
    /// ```
    /// let high = vec![200.0, 210.0, 205.0, 190.0];
    /// let low = vec![175.0, 192.0, 200.0, 174.0];
    /// let close = vec![192.0, 200.0, 201.0, 187.0];
    /// let williams_percent_r = rust_ti::momentum_indicators::bulk::williams_percent_r(&high, &low,
    /// &close);
    /// assert_eq!(vec![-32.0, -55.55555555555556, -80.0, -18.75], williams_percent_r);
    /// ```
    pub fn williams_percent_r(high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
        let length = close.len();
        if length != high.len() || length != low.len() {
            panic!(
                "Length of close ({}) needs to match length of high ({}), and length of close ({})",
                length,
                high.len(),
                close.len()
            );
        };

        let mut wprs = Vec::new();
        for i in 0..length {
            wprs.push(single::williams_percent_r(&high[i], &low[i], &close[i]));
        }
        return wprs;
    }

    /// The `money_flow_index` is a momentum indicator that shows the volume of money (a.k.a money
    /// flow) going in and out of the asset.
    ///
    /// The standard money flow index uses the typical price (high+low+close/3) as the observed
    /// price and a period of 14 days.
    ///
    /// # Arguments
    ///
    /// * `prices` - An `f64` slice of prices
    /// * `volume` - Volume of transactions
    /// * `period` - Period over which to calculate the money flow index
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0];
    /// let volume = vec![1000.0, 1500.0, 1200.0, 900.0, 1300.0];
    /// let period: usize = 3;
    /// let money_flow_index = rust_ti::momentum_indicators::bulk::money_flow_index(&prices,
    /// &volume, &period);
    /// assert_eq!(vec![55.314533622559644, 0.0, 58.60655737704918], money_flow_index);
    /// ```
    pub fn money_flow_index(prices: &[f64], volume: &[f64], period: &usize) -> Vec<f64> {
        let length = prices.len();
        if period > &length {
            panic!(
                "Period ({}) cannot be longer than length of prices ({})",
                period, length
            );
        };
        if length != volume.len() {
            panic!(
                "Length of prices ({}) must match length of volume ({})",
                length,
                volume.len()
            );
        };

        let mut mfis = Vec::new();
        let loop_max = length - period + 1;
        for i in 0..loop_max {
            mfis.push(single::money_flow_index(
                &prices[i..i + period],
                &volume[i..i + period],
            ));
        }
        return mfis;
    }

    /// The `rate_of_change` is a momentum indicator that shows how quickly the price of an asset
    /// is changing.
    ///
    /// The standard is to use the closing price.
    ///
    /// The bulk function only takes in a price and will loop through the slice and assume the
    /// prices\[0\] is the first previous price, and prices\[1\] is the first current price, and keep
    /// incrementing from there.
    ///
    /// # Arguments
    ///
    /// * `prices` - An `f64` slice of prices
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 120.0, 100.0];
    /// let rate_of_change = rust_ti::momentum_indicators::bulk::rate_of_change(&prices);
    /// assert_eq!(vec![20.0, -16.666666666666664], rate_of_change);
    /// ```
    pub fn rate_of_change(prices: &[f64]) -> Vec<f64> {
        if prices.is_empty() {
            panic!("Prices cannot be empty");
        }
        let mut rocs = Vec::new();
        for i in 1..prices.len() {
            rocs.push(single::rate_of_change(&prices[i], &prices[i - 1]));
        }
        return rocs;
    }

    /// The `on_balance_volume` is a momentum indicator that uses volume
    ///
    /// The standard price to use it the closing price. If there is no previous on balance volume
    /// pass in 0.
    ///
    /// # Arguments
    ///
    /// * `prices` - An `f64` slice of prices
    /// * `volume` - An `i64` slice of volumes
    /// * `previous_on_balance_volume` - Previous on balance volume. Use 0 if no previous OBV.
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0, 99.0];
    /// let volume = vec![1000, 1500, 1200, 900, 1300, 1400];
    /// let on_balance_volume =
    /// rust_ti::momentum_indicators::bulk::on_balance_volume(&prices, &volume, &0);
    /// assert_eq!(vec![1500, 2700, 1800, 500, 500], on_balance_volume);
    /// ```
    pub fn on_balance_volume(
        prices: &[f64],
        volume: &[i64],
        previous_on_balance_volume: &i64,
    ) -> Vec<i64> {
        if prices.is_empty() {
            panic!("Prices cannot be empty");
        };
        let length = prices.len();
        if length != volume.len() {
            panic!(
                "Length of prices ({}) and volume ({}) must match",
                length,
                volume.len()
            );
        };

        let mut obvs = vec![single::on_balance_volume(
            &prices[1],
            &prices[0],
            &volume[1],
            previous_on_balance_volume,
        )];
        for i in 2..length {
            obvs.push(single::on_balance_volume(
                &prices[i],
                &prices[i - 1],
                &volume[i],
                &obvs.last().unwrap(),
            ));
        }
        return obvs;
    }

    /// The `commodity_channel_index` is a momentum indicator flagging overbought and oversold
    /// conditions.
    ///
    /// The standard commodity channel index uses the typical price, a simple moving average model,
    /// and a mean absolute deviation model. The `constant_multiplier` is a scale factor used to
    /// provide more readable numbers. It is recommended to use 0.015 as the default as changing it
    /// will impact how the numbers are distributed and the standard -100/100 flags may no longer
    /// apply.
    ///
    /// # Arguments
    ///
    /// `prices` - An `f64` slice of prices
    /// `constant_model_type` - A variant of `ConstantModelType`
    /// `deviation_model` - A variant of `DeviationModel`
    /// `constant_multiplier` - Scale factor. Normally 0.015
    /// `period` - Period over which to calculate the commodity channel index
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0];
    /// let constant_multiplier = 0.015;
    /// let period: usize = 3;
    ///
    /// let default_cci = rust_ti::momentum_indicators::bulk::commodity_channel_index(&prices, &rust_ti::ConstantModelType::SimpleMovingAverage, &rust_ti::DeviationModel::MeanAbsoluteDeviation, &constant_multiplier, &period);
    /// assert_eq!(vec![79.99999999999983, -100.00000000000001, -100.00000000000001], default_cci);
    ///
    /// let ema_sd_cci = rust_ti::momentum_indicators::bulk::commodity_channel_index(&prices,
    /// &rust_ti::ConstantModelType::ExponentialMovingAverage,
    /// &rust_ti::DeviationModel::StandardDeviation, &constant_multiplier, &period);
    /// assert_eq!(vec![38.1801774160603, -58.32118435197994, -46.65694748158418], ema_sd_cci);
    ///
    /// let median_mad_cci = rust_ti::momentum_indicators::bulk::commodity_channel_index(&prices,
    /// &rust_ti::ConstantModelType::SimpleMovingMedian,
    /// &rust_ti::DeviationModel::MedianAbsoluteDeviation, &constant_multiplier, &period);  
    /// assert_eq!(vec![66.66666666666667, -100.00000000000001, -100.00000000000001], median_mad_cci);
    /// ```
    pub fn commodity_channel_index(
        prices: &[f64],
        constant_model_type: &crate::ConstantModelType,
        deviation_model: &crate::DeviationModel,
        constant_multiplier: &f64,
        period: &usize,
    ) -> Vec<f64> {
        let length = prices.len();
        if period > &length {
            panic!(
                "Period ({}) cannot be longer than lenght ({}) of prices",
                period, length
            );
        };

        let mut ccis = Vec::new();
        let loop_max = length - period + 1;
        for i in 0..loop_max {
            ccis.push(single::commodity_channel_index(
                &prices[i..i + period],
                constant_model_type,
                deviation_model,
                constant_multiplier,
            ));
        }
        return ccis;
    }

    /// The `mcginley_dynamic_commodity_channel_index` is a momentum indicator flagging overbought and oversold
    /// conditions.
    ///
    /// The McGinley dynamic commodity channel index uses the McGinley dynamic rather than a moving average model, and returns the CCI as well as the McGinley dynamic.
    /// The caller still needs to determine the absolute deviation model.
    ///
    /// # Arguments
    ///
    /// * `prices` - An `f64` slice of prices
    /// * `previous_mcginley_dynamic` - The previous value of the McGinley dynamic. 0.0 if no
    /// previous.
    /// * `deviation_model` - A variant of `DeviationModel`
    /// * `constant_multiplier` - Scale factor. Normally 0.015
    /// * `period` - The period over which to calculate the commodity channel index
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0, 102.0];
    /// let constant_multiplier = 0.015;
    ///
    /// let mcginley_cci = rust_ti::momentum_indicators::bulk::mcginley_dynamic_commodity_channel_index(
    /// &prices, &0.0, &rust_ti::DeviationModel::MeanAbsoluteDeviation, &constant_multiplier,
    /// &5_usize);
    /// assert_eq!(vec![(0.0, 99.0), (146.8770632107927, 99.53246533805869)], mcginley_cci);
    /// ```
    pub fn mcginley_dynamic_commodity_channel_index(
        prices: &[f64],
        previous_mcginley_dynamic: &f64,
        deviation_model: &crate::DeviationModel,
        constant_multiplier: &f64,
        period: &usize,
    ) -> Vec<(f64, f64)> {
        let length = prices.len();
        if period > &length {
            panic!(
                "Period ({}) cannot be longer the length of prices ({})",
                period, length
            );
        };

        let mut ccis = vec![single::mcginley_dynamic_commodity_channel_index(
            &prices[..*period],
            previous_mcginley_dynamic,
            deviation_model,
            constant_multiplier,
        )];
        let loop_max = length - period + 1;
        for i in 1..loop_max {
            let previous_dynamic = ccis[i - 1].1;
            ccis.push(single::mcginley_dynamic_commodity_channel_index(
                &prices[i..i + period],
                &previous_dynamic,
                deviation_model,
                constant_multiplier,
            ));
        }
        return ccis;
    }

    /// The `macd_line` is a momentum indicator that also shows price
    /// strength, direction, and general trend.
    ///
    /// The indicator was developer back when there was a 6 day trading week so the standard short
    /// term to use is 12, and 26 for the long term. However the caller can determine the period.
    /// Both tyically use an exponential moving average model, but this is also determined by the
    /// caller.
    ///
    /// The long period is determined by the length of the `prices` slice.
    ///
    /// This function *only* calculates the MACD line not the signal line, `signal_line` should be
    /// called for the signal line.
    ///
    /// # Arguments
    ///
    /// * `prices` - An `f64` slice of prices
    /// * `short_period` - The length of the short period
    /// * `short_period_model` - A variant of `ConstantModelType`
    /// * `long_period` - The length of the long period
    /// * `long_period_model` - A variant of `ConstantModelType`
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0, 99.0, 102.0];
    /// let short_period: usize = 3;
    /// let long_period: usize = 5;
    ///
    /// let macd = rust_ti::momentum_indicators::bulk::macd_line(&prices, &short_period,
    /// &rust_ti::ConstantModelType::ExponentialMovingAverage, &long_period,
    /// &rust_ti::ConstantModelType::ExponentialMovingAverage);
    /// assert_eq!(vec![-0.46851726472581845, -0.7379823967501835, 0.031821259309410266], macd);
    ///
    /// let macd = rust_ti::momentum_indicators::bulk::macd_line(&prices, &short_period,
    /// &rust_ti::ConstantModelType::SimpleMovingAverage, &long_period,
    /// &rust_ti::ConstantModelType::SimpleMovingMedian);
    /// assert_eq!(vec![0.0, -1.3333333333333286, -1.0], macd);
    /// ```
    pub fn macd_line(
        prices: &[f64],
        short_period: &usize,
        short_period_model: &crate::ConstantModelType,
        long_period: &usize,
        long_period_model: &crate::ConstantModelType,
    ) -> Vec<f64> {
        if short_period > long_period {
            panic!(
                "Short period ({}) cannot be longer than long period ({})",
                short_period, long_period
            )
        };

        let length = prices.len();
        if long_period > &length {
            panic!(
                "Long period ({}) cannot be shorter than length of prices ({})",
                long_period, length
            );
        };

        let mut macds = Vec::new();
        let loop_max = length - long_period + 1;
        for i in 0..loop_max {
            macds.push(single::macd_line(
                &prices[i..i + long_period],
                short_period,
                short_period_model,
                long_period_model,
            ));
        }
        return macds;
    }

    /// The `signal_line` is used with the `macd_line` to produce the moving average convergence
    /// divergence.
    ///
    /// The standard period for the signal line is 9, and it normally uses an exponential moving
    /// average model.
    ///
    /// # Arguments
    ///
    /// * `macds` - An `f64` slice of MACDs
    /// * constant_model_type` - A variant of `ConstantModelType`
    /// * `period` - Period over which to calculate the signal line
    ///
    /// # Examples
    ///
    /// ```
    /// let macds = vec![-0.0606702775897219, -0.0224170616113781, 0.0057887610020515,
    /// 0.0318212593094103, -0.737982396750169];
    /// let period: usize = 3;
    ///
    /// let ema_signal_line = rust_ti::momentum_indicators::bulk::signal_line(&macds,
    /// &rust_ti::ConstantModelType::ExponentialMovingAverage, &period);
    /// assert_eq!(vec![-0.011764193829181728, 0.0166350710900523, -0.4117854724828291], ema_signal_line);
    ///
    /// let median_signal_line = rust_ti::momentum_indicators::bulk::signal_line(&macds,
    /// &rust_ti::ConstantModelType::SimpleMovingMedian, &period);
    /// assert_eq!(vec![-0.0224170616113781, 0.0057887610020515, 0.0057887610020515], median_signal_line);
    /// ```
    pub fn signal_line(
        macds: &[f64],
        constant_model_type: &crate::ConstantModelType,
        period: &usize,
    ) -> Vec<f64> {
        let length = macds.len();
        if period > &length {
            panic!(
                "Period ({}) cannot be longer than length ({}) of prices",
                period, length
            )
        };

        let mut signals = Vec::new();
        let loop_max = length - period + 1;
        for i in 0..loop_max {
            signals.push(single::signal_line(
                &macds[i..i + period],
                constant_model_type,
            ));
        }
        return signals;
    }

    /// The `mcginley_dynamic_macd_line` is an alternative to `macd_line` that uses and returns the
    /// McGinley Dynamic as well as the MACD line.
    ///
    /// The function returns a tuple with the MACD value first, short period McGinley dynamic second, and the long period third.
    ///
    /// The is no McGinley dynamic just for the signal line as the singal line merely takes a moving
    /// average of the MACDs, for a McGinley dynamic version just call
    /// `mcginley_dynamic` from `moving_averages.rs`
    ///
    /// # Arguments
    ///
    /// * `prices` - An `f64` slice of prices
    /// * `short_period` - The length of the short period
    /// * `previous_short_mcginley` - Previous McGinley dynamic for the short model. If no
    /// previous use 0.0.
    /// * `long_period` - The length of the long period
    /// * `previous_long_mcginley` - Previous McGinley dynamic for the long model. If no
    /// previous use 0.0.
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0, 99.0, 102.0];
    /// let short_period: usize = 3;
    /// let long_period: usize = 5;
    /// let mcginley_dynamic_macd = rust_ti::momentum_indicators::bulk::mcginley_dynamic_macd_line(
    /// &prices, &short_period, &0.0, &long_period, &0.0);
    /// assert_eq!(vec![(0.0, 99.0, 99.0), (0.0, 99.0, 99.0), (0.35497689203913296,
    /// 99.88744223009782, 99.53246533805869)], mcginley_dynamic_macd);
    /// ```
    pub fn mcginley_dynamic_macd_line(
        prices: &[f64],
        short_period: &usize,
        previous_short_mcginley: &f64,
        long_period: &usize,
        previous_long_mcginley: &f64,
    ) -> Vec<(f64, f64, f64)> {
        if prices.is_empty() {
            panic!("Prices cannot be empty");
        };

        let length = prices.len();
        if long_period > &length {
            panic!(
                "Long period ({}) cannot be longer than length ({}) of prices",
                long_period, length
            );
        };

        let mut macds = vec![single::mcginley_dynamic_macd_line(
            &prices[..*long_period],
            short_period,
            previous_short_mcginley,
            previous_long_mcginley,
        )];
        let loop_max = length - long_period + 1;
        for i in 1..loop_max {
            let previous_macd = macds[i - 1];
            macds.push(single::mcginley_dynamic_macd_line(
                &prices[i..i + long_period],
                short_period,
                &previous_macd.1,
                &previous_macd.2,
            ));
        }
        return macds;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_ma_rsi() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(
            49.2537313432832,
            single::relative_strength_index(
                &prices,
                &crate::ConstantModelType::SimpleMovingAverage
            )
        );
    }

    #[test]
    fn test_single_short_median_rsi() {
        // Because there are too few values, ends up being the means
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(
            49.2537313432832,
            single::relative_strength_index(&prices, &crate::ConstantModelType::SimpleMovingMedian)
        );
    }

    #[test]
    fn test_single_long_median_rsi() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        assert_eq!(
            37.5,
            single::relative_strength_index(&prices, &crate::ConstantModelType::SimpleMovingMedian)
        );
    }

    #[test]
    fn test_single_small_mode_rsi() {
        // Mode rounds the values, the difference being so small all rounds down to 0.0
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(
            0.0,
            single::relative_strength_index(&prices, &crate::ConstantModelType::SimpleMovingMode)
        );
    }

    #[test]
    fn test_single_large_mode_rsi() {
        let prices = vec![100.0, 103.0, 106.0, 107.0, 108.0, 105.0, 102.0];
        assert_eq!(
            39.99999999999999,
            single::relative_strength_index(&prices, &crate::ConstantModelType::SimpleMovingMode)
        );
    }

    #[test]
    fn test_single_smoothed_rsi() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(
            43.01075268817234,
            single::relative_strength_index(
                &prices,
                &crate::ConstantModelType::SmoothedMovingAverage
            )
        );
    }

    #[test]
    fn test_single_exponential_rsi() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(
            39.495798319328436,
            single::relative_strength_index(
                &prices,
                &crate::ConstantModelType::ExponentialMovingAverage
            )
        );
    }

    #[test]
    fn test_single_personalised_rsi() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(
            35.6725146198842,
            single::relative_strength_index(
                &prices,
                &crate::ConstantModelType::PersonalisedMovingAverage(&4.0, &3.0)
            )
        );
    }

    #[test]
    fn test_single_only_price_rise_rsi() {
        let prices = vec![100.0, 101.0, 102.0, 103.0];
        assert_eq!(
            100.0,
            single::relative_strength_index(
                &prices,
                &crate::ConstantModelType::SimpleMovingAverage
            )
        );
    }

    #[test]
    fn test_single_only_price_fall_rsi() {
        let prices = vec![103.0, 102.0, 101.0, 100.0];
        assert_eq!(
            0.0,
            single::relative_strength_index(
                &prices,
                &crate::ConstantModelType::SimpleMovingAverage
            )
        );
    }

    #[test]
    #[should_panic]
    fn test_single_rsi_panic() {
        let prices = Vec::new();
        single::relative_strength_index(&prices, &crate::ConstantModelType::SimpleMovingAverage);
    }

    #[test]
    fn test_bulk_simple_ma_rsi() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        let period: usize = 5;
        assert_eq!(
            vec![
                49.2537313432832,
                20.930232558140005,
                27.6595744680842,
                36.111111111111335
            ],
            bulk::relative_strength_index(
                &prices,
                &crate::ConstantModelType::SimpleMovingAverage,
                &period
            )
        );
    }

    #[test]
    fn test_bulk_smoothed_ma_rsi() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        let period: usize = 5;
        assert_eq!(
            vec![
                43.01075268817234,
                17.187499999999886,
                31.168831168830664,
                47.05882352941291
            ],
            bulk::relative_strength_index(
                &prices,
                &crate::ConstantModelType::SmoothedMovingAverage,
                &period
            )
        );
    }

    #[test]
    fn test_bulk_exponential_ma_rsi() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        let period: usize = 5;
        assert_eq!(
            vec![
                39.495798319328436,
                15.2941176470584,
                32.71028037383145,
                53.03030303030472
            ],
            bulk::relative_strength_index(
                &prices,
                &crate::ConstantModelType::ExponentialMovingAverage,
                &period
            )
        );
    }

    #[test]
    fn test_bulk_personalised_ma_rsi() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        let period: usize = 5;
        assert_eq!(
            vec![
                35.6725146198842,
                13.385826771652745,
                34.13173652694594,
                59.375000000002316
            ],
            bulk::relative_strength_index(
                &prices,
                &crate::ConstantModelType::PersonalisedMovingAverage(&4.0, &3.0),
                &period
            )
        );
    }

    #[test]
    fn test_bulk_simple_median_rsi() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        let period: usize = 5;
        assert_eq!(
            vec![
                49.2537313432832,
                20.930232558140005,
                27.6595744680842,
                36.111111111111335
            ],
            bulk::relative_strength_index(
                &prices,
                &crate::ConstantModelType::SimpleMovingMedian,
                &period
            )
        );
    }

    #[test]
    fn test_bulk_simple_mode_rsi() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        let period: usize = 5;
        assert_eq!(
            vec![0.0, 0.0, 0.0, 0.0],
            bulk::relative_strength_index(
                &prices,
                &crate::ConstantModelType::SimpleMovingMode,
                &period
            )
        );
    }

    #[test]
    #[should_panic]
    fn test_bulk_rsi_panic() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        let period: usize = 50;
        bulk::relative_strength_index(
            &prices,
            &crate::ConstantModelType::SimpleMovingAverage,
            &period,
        );
    }

    #[test]
    fn test_mcginley_dynamic_rsi_no_previous() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(
            (26.923076923079236, 0.07000000000000739, 0.18999999999999773),
            single::mcginley_dynamic_rsi(&prices, &0.0, &0.0)
        );
    }

    #[test]
    fn test_mcginley_dynamic_rsi_previous() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            (105.44168978787299, -3.6815625000054153, 0.18999999999999773),
            single::mcginley_dynamic_rsi(&prices, &0.07000000000000739, &0.18999999999999773)
        );
    }

    #[test]
    #[should_panic]
    fn test_mcginley_dynamic_rsi_panic() {
        let prices = Vec::new();
        single::mcginley_dynamic_rsi(&prices, &0.0, &0.0);
    }

    #[test]
    fn test_bulk_mcginley_dynamic_rsi_no_previous() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        let period: usize = 5;
        assert_eq!(
            vec![
                (26.923076923079236, 0.07000000000000739, 0.18999999999999773),
                (105.44168978787299, -3.6815625000054153, 0.18999999999999773),
                (99.99999201255198, 2378732.0357780023, 0.18999999999999773),
                (100.0, -2.6009191432509403e35, -37.989980468780004)
            ],
            bulk::mcginley_dynamic_rsi(&prices, &0.0, &0.0, &period)
        );
    }

    #[test]
    fn test_bulk_mcginley_dynamic_rsi_previous() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        let period: usize = 5;
        assert_eq!(
            vec![
                (105.44168978787299, -3.6815625000054153, 0.18999999999999773),
                (99.99999201255198, 2378732.0357780023, 0.18999999999999773),
                (100.0, -2.6009191432509403e35, -37.989980468780004)
            ],
            bulk::mcginley_dynamic_rsi(
                &prices,
                &0.07000000000000739,
                &0.18999999999999773,
                &period
            )
        );
    }

    #[test]
    #[should_panic]
    fn test_bulk_mcginley_dynamic_rsi_panic() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        let period: usize = 50;
        bulk::mcginley_dynamic_rsi(&prices, &0.0, &0.0, &period);
    }

    #[test]
    fn test_single_stochastic_oscillator_min() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(0.0, single::stochastic_oscillator(&prices));
    }

    #[test]
    fn test_single_stochastic_oscillator_max() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.66];
        assert_eq!(100.0, single::stochastic_oscillator(&prices));
    }

    #[test]
    fn test_single_stochastic_oscillator() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.34];
        assert_eq!(42.42424242424281, single::stochastic_oscillator(&prices));
    }

    #[test]
    #[should_panic]
    fn test_single_stochastic_oscillator_panic() {
        let prices = Vec::new();
        single::stochastic_oscillator(&prices);
    }

    #[test]
    fn test_bulk_stochastic_oscillator() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        let period: usize = 5;
        assert_eq!(
            vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394],
            bulk::stochastic_oscillator(&prices, &period)
        );
    }

    #[test]
    #[should_panic]
    fn test_bulk_stochastic_oscillator_bulk() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        let period: usize = 50;
        bulk::stochastic_oscillator(&prices, &period);
    }

    #[test]
    fn test_single_ma_slow_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        assert_eq!(
            22.871517027863632,
            single::slow_stochastic(&stochastics, &crate::ConstantModelType::SimpleMovingAverage)
        );
    }

    #[test]
    fn test_single_sma_slow_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        assert_eq!(
            29.02078726227347,
            single::slow_stochastic(
                &stochastics,
                &crate::ConstantModelType::SmoothedMovingAverage
            )
        );
    }

    #[test]
    fn test_single_ema_slow_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        assert_eq!(
            33.284579311601206,
            single::slow_stochastic(
                &stochastics,
                &crate::ConstantModelType::ExponentialMovingAverage
            )
        );
    }

    #[test]
    fn test_single_pma_slow_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        assert_eq!(
            39.872151259403616,
            single::slow_stochastic(
                &stochastics,
                &crate::ConstantModelType::PersonalisedMovingAverage(&5.0, &4.0)
            )
        );
    }

    #[test]
    fn test_single_median_slow_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        assert_eq!(
            22.05882352941029,
            single::slow_stochastic(&stochastics, &crate::ConstantModelType::SimpleMovingMedian)
        );
    }

    #[test]
    fn test_single_mode_slow_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        assert_eq!(
            22.75,
            single::slow_stochastic(&stochastics, &crate::ConstantModelType::SimpleMovingMode)
        );
    }

    #[test]
    #[should_panic]
    fn test_single_slow_stochastic_panic() {
        let stochastics = Vec::new();
        single::slow_stochastic(&stochastics, &crate::ConstantModelType::SimpleMovingMode);
    }

    #[test]
    fn test_bulk_ma_slow_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 3;
        assert_eq!(
            vec![14.705882352940193, 30.49535603715151],
            bulk::slow_stochastic(
                &stochastics,
                &crate::ConstantModelType::SimpleMovingAverage,
                &period
            )
        );
    }

    #[test]
    fn test_bulk_sma_slow_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 3;
        assert_eq!(
            vec![19.969040247676816, 35.75036662864623],
            bulk::slow_stochastic(
                &stochastics,
                &crate::ConstantModelType::SmoothedMovingAverage,
                &period
            )
        );
    }

    #[test]
    fn test_bulk_ema_slow_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 3;
        assert_eq!(
            vec![23.529411764704548, 38.832375055285944],
            bulk::slow_stochastic(
                &stochastics,
                &crate::ConstantModelType::ExponentialMovingAverage,
                &period
            )
        );
    }

    #[test]
    fn test_bulk_pma_slow_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 3;
        assert_eq!(
            vec![29.192273924493655, 42.98322628344476],
            bulk::slow_stochastic(
                &stochastics,
                &crate::ConstantModelType::PersonalisedMovingAverage(&5.0, &4.0),
                &period
            )
        );
    }

    #[test]
    fn test_bulk_median_slow_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 3;
        assert_eq!(
            vec![5.882352941175241, 38.23529411764534],
            bulk::slow_stochastic(
                &stochastics,
                &crate::ConstantModelType::SimpleMovingMedian,
                &period
            )
        );
    }

    #[test]
    fn test_bulk_mode_slow_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 3;
        assert_eq!(
            vec![14.666666666666666, 30.3333333333333332],
            bulk::slow_stochastic(
                &stochastics,
                &crate::ConstantModelType::SimpleMovingMode,
                &period
            )
        );
    }

    #[test]
    #[should_panic]
    fn test_bulk_slow_stochastic_panic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 30;
        bulk::slow_stochastic(
            &stochastics,
            &crate::ConstantModelType::SimpleMovingMode,
            &period,
        );
    }

    #[test]
    fn test_single_ma_slowest_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        assert_eq!(
            22.871517027863632,
            single::slowest_stochastic(
                &stochastics,
                &crate::ConstantModelType::SimpleMovingAverage
            )
        );
    }

    #[test]
    fn test_single_sma_slowest_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        assert_eq!(
            29.02078726227347,
            single::slowest_stochastic(
                &stochastics,
                &crate::ConstantModelType::SmoothedMovingAverage
            )
        );
    }

    #[test]
    fn test_single_ema_slowest_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        assert_eq!(
            33.284579311601206,
            single::slowest_stochastic(
                &stochastics,
                &crate::ConstantModelType::ExponentialMovingAverage
            )
        );
    }

    #[test]
    fn test_single_pma_slowest_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        assert_eq!(
            39.872151259403616,
            single::slowest_stochastic(
                &stochastics,
                &crate::ConstantModelType::PersonalisedMovingAverage(&5.0, &4.0)
            )
        );
    }

    #[test]
    fn test_single_median_slowest_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        assert_eq!(
            22.05882352941029,
            single::slowest_stochastic(&stochastics, &crate::ConstantModelType::SimpleMovingMedian)
        );
    }

    #[test]
    fn test_single_mode_slowest_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        assert_eq!(
            22.75,
            single::slowest_stochastic(&stochastics, &crate::ConstantModelType::SimpleMovingMode)
        );
    }

    #[test]
    #[should_panic]
    fn test_single_slowest_stochastic_panic() {
        let stochastics = Vec::new();
        single::slowest_stochastic(&stochastics, &crate::ConstantModelType::SimpleMovingMode);
    }

    #[test]
    fn test_bulk_ma_slowest_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 3;
        assert_eq!(
            vec![14.705882352940193, 30.49535603715151],
            bulk::slowest_stochastic(
                &stochastics,
                &crate::ConstantModelType::SimpleMovingAverage,
                &period
            )
        );
    }

    #[test]
    fn test_bulk_sma_slowest_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 3;
        assert_eq!(
            vec![19.969040247676816, 35.75036662864623],
            bulk::slowest_stochastic(
                &stochastics,
                &crate::ConstantModelType::SmoothedMovingAverage,
                &period
            )
        );
    }

    #[test]
    fn test_bulk_ema_slowest_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 3;
        assert_eq!(
            vec![23.529411764704548, 38.832375055285944],
            bulk::slowest_stochastic(
                &stochastics,
                &crate::ConstantModelType::ExponentialMovingAverage,
                &period
            )
        );
    }

    #[test]
    fn test_bulk_pma_slowest_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 3;
        assert_eq!(
            vec![29.192273924493655, 42.98322628344476],
            bulk::slowest_stochastic(
                &stochastics,
                &crate::ConstantModelType::PersonalisedMovingAverage(&5.0, &4.0),
                &period
            )
        );
    }

    #[test]
    fn test_bulk_median_slowest_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 3;
        assert_eq!(
            vec![5.882352941175241, 38.23529411764534],
            bulk::slowest_stochastic(
                &stochastics,
                &crate::ConstantModelType::SimpleMovingMedian,
                &period
            )
        );
    }

    #[test]
    fn test_bulk_mode_slowest_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 3;
        assert_eq!(
            vec![14.666666666666666, 30.3333333333333332],
            bulk::slowest_stochastic(
                &stochastics,
                &crate::ConstantModelType::SimpleMovingMode,
                &period
            )
        );
    }

    #[test]
    #[should_panic]
    fn test_bulk_slowest_stochastic_panic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 30;
        bulk::slowest_stochastic(
            &stochastics,
            &crate::ConstantModelType::SimpleMovingMode,
            &period,
        );
    }

    #[test]
    fn test_single_williams_percent_r() {
        let high = 100.93;
        let low = 100.37;
        let close = 100.49;
        assert_eq!(
            -78.57142857143037,
            single::williams_percent_r(&high, &low, &close)
        );
    }

    #[test]
    fn test_bulk_williams_percent_r() {
        let high = vec![100.93, 101.58, 101.25];
        let low = vec![100.37, 100.57, 100.94];
        let close = vec![100.49, 101.06, 101.13];
        assert_eq!(
            vec![-78.57142857143037, -51.485148514850835, -38.70967741935602],
            bulk::williams_percent_r(&high, &low, &close)
        );
    }

    #[test]
    #[should_panic]
    fn test_bulk_williams_percent_r_high_panic() {
        let high = vec![101.58, 101.25];
        let low = vec![100.37, 100.57, 100.94];
        let close = vec![100.49, 101.06, 101.13];
        bulk::williams_percent_r(&high, &low, &close);
    }

    #[test]
    #[should_panic]
    fn test_bulk_williams_percent_r_low_panic() {
        let high = vec![100.93, 101.58, 101.25];
        let low = vec![100.37, 100.57, 100.94, 100.59];
        let close = vec![100.49, 101.06, 101.13];
        bulk::williams_percent_r(&high, &low, &close);
    }

    #[test]
    fn test_single_money_flow_index() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        let volume = vec![1200.0, 1400.0, 1450.0, 1100.0, 900.0, 875.0, 1025.0, 1100.0];
        assert_eq!(
            63.40886336843541,
            single::money_flow_index(&prices, &volume)
        );
    }

    #[test]
    fn test_single_money_flow_index_only_negative() {
        let prices = vec![100.38, 100.19, 100.12];
        let volume = vec![1100.0, 900.0, 875.0];
        assert_eq!(0.0, single::money_flow_index(&prices, &volume));
    }

    #[test]
    fn test_single_money_flow_index_only_positive() {
        let prices = vec![100.2, 100.46, 100.53];
        let volume = vec![1200.0, 1400.0, 1450.0];
        assert_eq!(100.0, single::money_flow_index(&prices, &volume));
    }

    #[test]
    #[should_panic]
    fn test_single_money_flow_index_panic() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        let volume = vec![1200.0, 1400.0, 1450.0, 1100.0, 900.0, 875.0, 1025.0, 1100.0];
        single::money_flow_index(&prices, &volume);
    }

    #[test]
    #[should_panic]
    fn test_single_money_flow_index_empty_price_panic() {
        let prices = Vec::new();
        let volume = vec![1200.0, 1400.0, 1450.0, 1100.0, 900.0, 875.0, 1025.0, 1100.0];
        single::money_flow_index(&prices, &volume);
    }

    #[test]
    #[should_panic]
    fn test_single_money_flow_index_empty_volume_panic() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        let volume = Vec::new();
        single::money_flow_index(&prices, &volume);
    }

    #[test]
    fn test_bulk_money_flow_index() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        let volume = vec![1200.0, 1400.0, 1450.0, 1100.0, 900.0, 875.0, 1025.0, 1100.0];
        let period: usize = 5;
        assert_eq!(
            vec![
                58.811420498704834,
                33.5840199520207,
                26.291946512503486,
                54.5117755343317
            ],
            bulk::money_flow_index(&prices, &volume, &period)
        );
    }

    #[test]
    #[should_panic]
    fn test_bulk_money_flow_index_length_panic() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        let volume = vec![1400.0, 1450.0, 1100.0, 900.0, 875.0, 1025.0, 1100.0];
        let period: usize = 5;
        bulk::money_flow_index(&prices, &volume, &period);
    }

    #[test]
    #[should_panic]
    fn test_bulk_money_flow_index_period_panic() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        let volume = vec![1200.0, 1400.0, 1450.0, 1100.0, 900.0, 875.0, 1025.0, 1100.0];
        let period: usize = 50;
        bulk::money_flow_index(&prices, &volume, &period);
    }

    #[test]
    fn test_single_rate_of_change_positive() {
        let current_price = 100.46;
        let previous_price = 100.2;
        assert_eq!(
            0.25948103792414257,
            single::rate_of_change(&current_price, &previous_price)
        );
    }

    #[test]
    fn test_single_rate_of_change_negative() {
        let current_price = 100.19;
        let previous_price = 100.38;
        assert_eq!(
            -0.18928073321378536,
            single::rate_of_change(&current_price, &previous_price)
        );
    }

    #[test]
    fn test_single_rate_of_change_equal() {
        let current_price = 100.32;
        let previous_price = 100.32;
        assert_eq!(0.0, single::rate_of_change(&current_price, &previous_price));
    }

    #[test]
    fn test_bulk_rate_of_change() {
        let prices = vec![100.2, 100.46, 100.38, 100.19, 100.32, 100.32];
        assert_eq!(
            vec![
                0.25948103792414257,
                -0.07963368504877394,
                -0.18928073321378536,
                0.12975346841001642,
                0.0
            ],
            bulk::rate_of_change(&prices)
        );
    }

    #[test]
    #[should_panic]
    fn test_bulk_rate_of_change_panic() {
        let prices = Vec::new();
        bulk::rate_of_change(&prices);
    }

    #[test]
    fn test_single_on_balance_volume_positive_no_previous() {
        let current_price = 100.46;
        let previous_price = 100.2;
        let volume = 1500;
        let previous_obv = 0;
        assert_eq!(
            1500,
            single::on_balance_volume(&current_price, &previous_price, &volume, &previous_obv)
        );
    }

    #[test]
    fn test_single_on_balance_volume_positive_previous() {
        let current_price = 100.46;
        let previous_price = 100.2;
        let volume = 1500;
        let previous_obv = 500;
        assert_eq!(
            2000,
            single::on_balance_volume(&current_price, &previous_price, &volume, &previous_obv)
        );
    }

    #[test]
    fn test_single_on_balance_volume_negative_no_previous() {
        let current_price = 100.19;
        let previous_price = 100.32;
        let volume = 1500;
        let previous_obv = 0;
        assert_eq!(
            -1500,
            single::on_balance_volume(&current_price, &previous_price, &volume, &previous_obv)
        );
    }

    #[test]
    fn test_single_on_balance_volume_negative_previous() {
        let current_price = 100.19;
        let previous_price = 100.38;
        let volume = 1500;
        let previous_obv = 500;
        assert_eq!(
            -1000,
            single::on_balance_volume(&current_price, &previous_price, &volume, &previous_obv)
        );
    }

    #[test]
    fn test_single_on_balance_volume_equal_no_previous() {
        let current_price = 100.32;
        let previous_price = 100.32;
        let volume = 1500;
        let previous_obv = 0;
        assert_eq!(
            0,
            single::on_balance_volume(&current_price, &previous_price, &volume, &previous_obv)
        );
    }

    #[test]
    fn test_single_on_balance_volume_equal_previous() {
        let current_price = 100.32;
        let previous_price = 100.32;
        let volume = 1500;
        let previous_obv = 500;
        assert_eq!(
            500,
            single::on_balance_volume(&current_price, &previous_price, &volume, &previous_obv)
        );
    }

    #[test]
    fn test_bulk_on_balance_volume_no_previous() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        let volume = vec![1400, 1450, 1100, 900, 875];
        assert_eq!(
            vec![1450, 350, -550, 325],
            bulk::on_balance_volume(&prices, &volume, &0)
        );
    }

    #[test]
    fn test_bulk_on_balance_volume_previous() {
        let prices = vec![100.53, 100.38, 100.19, 100.21];
        let volume = vec![1450, 1100, 900, 875];
        assert_eq!(
            vec![350, -550, 325],
            bulk::on_balance_volume(&prices, &volume, &1450)
        );
    }

    #[test]
    #[should_panic]
    fn test_bulk_on_balance_volume_no_price_panic() {
        let prices = Vec::new();
        let volume = vec![1400, 1450, 1100, 900, 875];
        bulk::on_balance_volume(&prices, &volume, &0);
    }

    #[test]
    #[should_panic]
    fn test_bulk_on_balance_volume_no_volume_panic() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        let volume = Vec::new();
        bulk::on_balance_volume(&prices, &volume, &0);
    }

    #[test]
    #[should_panic]
    fn test_single_commodity_channel_index_panic() {
        let prices = Vec::new();
        single::commodity_channel_index(
            &prices,
            &crate::ConstantModelType::SimpleMovingAverage,
            &crate::DeviationModel::MeanAbsoluteDeviation,
            &0.015,
        );
    }

    #[test]
    fn test_single_ma_mean_ad_commodity_channel_index() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            -77.92207792208092,
            single::commodity_channel_index(
                &prices,
                &crate::ConstantModelType::SimpleMovingAverage,
                &crate::DeviationModel::MeanAbsoluteDeviation,
                &0.015
            )
        );
    }

    #[test]
    fn test_single_ma_median_ad_commodity_channel_index() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            -81.35593220339244,
            single::commodity_channel_index(
                &prices,
                &crate::ConstantModelType::SimpleMovingAverage,
                &crate::DeviationModel::MedianAbsoluteDeviation,
                &0.015
            )
        );
    }

    #[test]
    fn test_single_ma_mode_ad_commodity_channel_index() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            -27.11864406779792,
            single::commodity_channel_index(
                &prices,
                &crate::ConstantModelType::SimpleMovingAverage,
                &crate::DeviationModel::ModeAbsoluteDeviation,
                &0.015
            )
        );
    }

    #[test]
    fn test_single_ma_std_dev_commodity_channel_index() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            -71.3483546791537,
            single::commodity_channel_index(
                &prices,
                &crate::ConstantModelType::SimpleMovingAverage,
                &crate::DeviationModel::StandardDeviation,
                &0.015
            )
        );
    }

    #[test]
    fn test_single_ma_ulcer_index_commodity_channel_index() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            -44.00422507932252,
            single::commodity_channel_index(
                &prices,
                &crate::ConstantModelType::SimpleMovingAverage,
                &crate::DeviationModel::UlcerIndex,
                &0.015
            )
        );
    }
    #[test]
    fn test_single_sma_mean_ad_commodity_channel_index() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            -57.79560753382917,
            single::commodity_channel_index(
                &prices,
                &crate::ConstantModelType::SmoothedMovingAverage,
                &crate::DeviationModel::MeanAbsoluteDeviation,
                &0.015
            )
        );
    }

    #[test]
    fn test_single_ema_mean_ad_commodity_channel_index() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            -42.87971112616663,
            single::commodity_channel_index(
                &prices,
                &crate::ConstantModelType::ExponentialMovingAverage,
                &crate::DeviationModel::MeanAbsoluteDeviation,
                &0.015
            )
        );
    }

    #[test]
    fn test_single_pma_mean_ad_commodity_channel_index() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            -19.132669714320674,
            single::commodity_channel_index(
                &prices,
                &crate::ConstantModelType::PersonalisedMovingAverage(&5.0, &4.0),
                &crate::DeviationModel::MeanAbsoluteDeviation,
                &0.015
            )
        );
    }

    #[test]
    fn test_single_median_mean_ad_commodity_channel_index() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            -91.99134199134296,
            single::commodity_channel_index(
                &prices,
                &crate::ConstantModelType::SimpleMovingMedian,
                &crate::DeviationModel::MeanAbsoluteDeviation,
                &0.015
            )
        );
    }

    #[test]
    fn test_single_mode_mean_ad_commodity_channel_index() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            113.63636363636031,
            single::commodity_channel_index(
                &prices,
                &crate::ConstantModelType::SimpleMovingMode,
                &crate::DeviationModel::MeanAbsoluteDeviation,
                &0.015
            )
        );
    }

    #[test]
    fn test_bulk_commodity_channel_index() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            vec![-100.0, -100.00000000000804, -41.66666666666519],
            bulk::commodity_channel_index(
                &prices,
                &crate::ConstantModelType::SimpleMovingAverage,
                &crate::DeviationModel::MeanAbsoluteDeviation,
                &0.015,
                &3_usize
            )
        );
    }

    #[test]
    #[should_panic]
    fn test_bulk_commodity_channel_index_panic() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            vec![-100.0, -100.00000000000804, -41.66666666666519],
            bulk::commodity_channel_index(
                &prices,
                &crate::ConstantModelType::SimpleMovingAverage,
                &crate::DeviationModel::MeanAbsoluteDeviation,
                &0.015,
                &30_usize
            )
        );
    }

    #[test]
    fn test_single_mcginley_cci_no_previous() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            (0.0, 100.21),
            single::mcginley_dynamic_commodity_channel_index(
                &prices,
                &0.0,
                &crate::DeviationModel::MeanAbsoluteDeviation,
                &0.015
            )
        );
    }

    #[test]
    fn test_single_mcginley_cci_previous_mean_absolute_deviation() {
        let prices = vec![100.53, 100.38, 100.19, 100.21, 100.32];
        assert_eq!(
            (56.90977560811997, 100.23190366735862),
            single::mcginley_dynamic_commodity_channel_index(
                &prices,
                &100.21,
                &crate::DeviationModel::MeanAbsoluteDeviation,
                &0.015
            )
        );
    }

    #[test]
    fn test_single_mcginley_cci_previous_median_absolute_deviation() {
        let prices = vec![100.53, 100.38, 100.19, 100.21, 100.32];
        assert_eq!(
            (57.57930237998023, 100.23190366735862),
            single::mcginley_dynamic_commodity_channel_index(
                &prices,
                &100.21,
                &crate::DeviationModel::MedianAbsoluteDeviation,
                &0.015
            )
        );
    }

    #[test]
    fn test_single_mcginley_cci_previous_mode_absolute_deviation() {
        let prices = vec![100.53, 100.38, 100.19, 100.21, 100.32];
        assert_eq!(
            (18.015609947110768, 100.23190366735862),
            single::mcginley_dynamic_commodity_channel_index(
                &prices,
                &100.21,
                &crate::DeviationModel::ModeAbsoluteDeviation,
                &0.015
            )
        );
    }

    #[test]
    fn test_single_mcginley_cci_previous_standard_deviation() {
        let prices = vec![100.53, 100.38, 100.19, 100.21, 100.32];
        assert_eq!(
            (47.47490364820863, 100.23190366735862),
            single::mcginley_dynamic_commodity_channel_index(
                &prices,
                &100.21,
                &crate::DeviationModel::StandardDeviation,
                &0.015
            )
        );
    }

    #[test]
    fn test_single_mcginley_cci_previous_ulcer_index() {
        let prices = vec![100.53, 100.38, 100.19, 100.21, 100.32];
        assert_eq!(
            (24.747413068246022, 100.23190366735862),
            single::mcginley_dynamic_commodity_channel_index(
                &prices,
                &100.21,
                &crate::DeviationModel::UlcerIndex,
                &0.015
            )
        );
    }

    #[test]
    #[should_panic]
    fn test_single_mcginley_cci_panic() {
        let prices = Vec::new();
        single::mcginley_dynamic_commodity_channel_index(
            &prices,
            &100.21,
            &crate::DeviationModel::MedianAbsoluteDeviation,
            &0.015,
        );
    }

    #[test]
    fn test_bulk_mcginley_cci_no_previous() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        assert_eq!(
            vec![
                (0.0, 100.21),
                (56.90977560811997, 100.23190366735862),
                (42.20998599356397, 100.24150449277387)
            ],
            bulk::mcginley_dynamic_commodity_channel_index(
                &prices,
                &0.0,
                &crate::DeviationModel::MeanAbsoluteDeviation,
                &0.015,
                &5_usize
            )
        );
    }

    #[test]
    fn test_bulk_mcginley_cci_previous() {
        let prices = vec![100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        assert_eq!(
            vec![
                (56.90977560811997, 100.23190366735862),
                (42.20998599356397, 100.24150449277387)
            ],
            bulk::mcginley_dynamic_commodity_channel_index(
                &prices,
                &100.21,
                &crate::DeviationModel::MeanAbsoluteDeviation,
                &0.015,
                &5_usize
            )
        );
    }

    #[test]
    #[should_panic]
    fn test_bulk_mcginley_cci_panic() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        bulk::mcginley_dynamic_commodity_channel_index(
            &prices,
            &0.0,
            &crate::DeviationModel::MeanAbsoluteDeviation,
            &0.015,
            &50_usize,
        );
    }

    #[test]
    fn test_single_ema_macd() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            -0.06067027758972188,
            single::macd_line(
                &prices,
                &3_usize,
                &crate::ConstantModelType::ExponentialMovingAverage,
                &crate::ConstantModelType::ExponentialMovingAverage
            )
        );
    }

    #[test]
    fn test_single_sma_macd() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            -0.07733259851198682,
            single::macd_line(
                &prices,
                &3_usize,
                &crate::ConstantModelType::SmoothedMovingAverage,
                &crate::ConstantModelType::SmoothedMovingAverage
            )
        );
    }

    #[test]
    fn test_single_pma_macd() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            -0.02938702437832319,
            single::macd_line(
                &prices,
                &3_usize,
                &crate::ConstantModelType::PersonalisedMovingAverage(&5.0, &4.0),
                &crate::ConstantModelType::PersonalisedMovingAverage(&5.0, &4.0)
            )
        );
    }

    #[test]
    fn test_single_ma_macd() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            -0.0940000000000083,
            single::macd_line(
                &prices,
                &3_usize,
                &crate::ConstantModelType::SimpleMovingAverage,
                &crate::ConstantModelType::SimpleMovingAverage
            )
        );
    }

    #[test]
    fn test_single_median_macd() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            -0.1700000000000017,
            single::macd_line(
                &prices,
                &3_usize,
                &crate::ConstantModelType::SimpleMovingMedian,
                &crate::ConstantModelType::SimpleMovingMedian
            )
        );
    }

    #[test]
    fn test_single_mode_macd() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            0.0,
            single::macd_line(
                &prices,
                &3_usize,
                &crate::ConstantModelType::SimpleMovingMode,
                &crate::ConstantModelType::SimpleMovingMode
            )
        );
    }

    #[test]
    #[should_panic]
    fn test_single_macd_panic() {
        let prices = Vec::new();
        single::macd_line(
            &prices,
            &3_usize,
            &crate::ConstantModelType::ExponentialMovingAverage,
            &crate::ConstantModelType::ExponentialMovingAverage,
        );
    }

    #[test]
    #[should_panic]
    fn test_single_macd_panic_period() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        single::macd_line(
            &prices,
            &30_usize,
            &crate::ConstantModelType::ExponentialMovingAverage,
            &crate::ConstantModelType::ExponentialMovingAverage,
        );
    }

    #[test]
    fn test_bulk_macd() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        assert_eq!(
            vec![
                -0.06067027758972188,
                -0.022417061611406552,
                0.005788761002008869
            ],
            bulk::macd_line(
                &prices,
                &3_usize,
                &crate::ConstantModelType::ExponentialMovingAverage,
                &5_usize,
                &crate::ConstantModelType::ExponentialMovingAverage
            )
        );
    }

    #[test]
    #[should_panic]
    fn test_bulk_macd_short_period_panic() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        bulk::macd_line(
            &prices,
            &30_usize,
            &crate::ConstantModelType::ExponentialMovingAverage,
            &5_usize,
            &crate::ConstantModelType::ExponentialMovingAverage,
        );
    }

    #[test]
    #[should_panic]
    fn test_bulk_macd_long_period_panic() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        bulk::macd_line(
            &prices,
            &3_usize,
            &crate::ConstantModelType::ExponentialMovingAverage,
            &50_usize,
            &crate::ConstantModelType::ExponentialMovingAverage,
        );
    }

    #[test]
    fn test_single_ema_signal() {
        let macds = vec![
            -0.06067027758972188,
            -0.022417061611406552,
            0.005788761002008869,
        ];
        assert_eq!(
            -0.011764193829214216,
            single::signal_line(&macds, &crate::ConstantModelType::ExponentialMovingAverage)
        );
    }

    #[test]
    fn test_single_sma_signal() {
        let macds = vec![
            -0.06067027758972188,
            -0.022417061611406552,
            0.005788761002008869,
        ];
        assert_eq!(
            -0.01710971742153932,
            single::signal_line(&macds, &crate::ConstantModelType::SmoothedMovingAverage)
        );
    }

    #[test]
    fn test_single_pma_signal() {
        let macds = vec![
            -0.06067027758972188,
            -0.022417061611406552,
            0.005788761002008869,
        ];
        assert_eq!(
            -0.004072696773434995,
            single::signal_line(
                &macds,
                &crate::ConstantModelType::PersonalisedMovingAverage(&5.0, &4.0)
            )
        );
    }

    #[test]
    fn test_single_ma_signal() {
        let macds = vec![
            -0.06067027758972188,
            -0.022417061611406552,
            0.005788761002008869,
        ];
        assert_eq!(
            -0.025766192733039855,
            single::signal_line(&macds, &crate::ConstantModelType::SimpleMovingAverage)
        );
    }

    #[test]
    fn test_single_median_signal() {
        let macds = vec![
            -0.06067027758972188,
            -0.022417061611406552,
            0.005788761002008869,
        ];
        assert_eq!(
            -0.022417061611406552,
            single::signal_line(&macds, &crate::ConstantModelType::SimpleMovingMedian)
        );
    }

    #[test]
    fn test_single_mode_signal() {
        let macds = vec![
            -0.06067027758972188,
            -0.022417061611406552,
            0.005788761002008869,
        ];
        assert_eq!(
            0.0,
            single::signal_line(&macds, &crate::ConstantModelType::SimpleMovingMode)
        );
    }

    #[test]
    #[should_panic]
    fn test_single_signal_panic() {
        let macds = Vec::new();
        single::signal_line(&macds, &crate::ConstantModelType::ExponentialMovingAverage);
    }

    #[test]
    #[should_panic]
    fn test_bulk_signal_panic() {
        let macds = vec![
            -0.06067027758972188,
            -0.022417061611406552,
            0.005788761002008869,
        ];
        bulk::signal_line(
            &macds,
            &crate::ConstantModelType::ExponentialMovingAverage,
            &30_usize,
        );
    }

    #[test]
    fn test_single_mcginley_macd_no_previous() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            (0.0, 100.21, 100.21),
            single::mcginley_dynamic_macd_line(&prices, &3_usize, &0.0, &0.0)
        );
    }

    #[test]
    fn test_single_mcginley_macd_previous() {
        let prices = vec![100.53, 100.38, 100.19, 100.21, 100.32];
        assert_eq!(
            (0.014602444905747802, 100.24650611226437, 100.23190366735862),
            single::mcginley_dynamic_macd_line(&prices, &3_usize, &100.21, &100.21)
        );
    }

    #[test]
    #[should_panic]
    fn test_single_mcginley_macd_panic_short_period() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        single::mcginley_dynamic_macd_line(&prices, &30_usize, &0.0, &0.0);
    }

    #[test]
    #[should_panic]
    fn test_single_mcginley_macd_panic_no_prices() {
        let prices = Vec::new();
        single::mcginley_dynamic_macd_line(&prices, &3_usize, &0.0, &0.0);
    }

    #[test]
    fn test_bulk_mcginley_macd_no_previous() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        assert_eq!(
            vec![
                (0.0, 100.21, 100.21),
                (0.014602444905747802, 100.24650611226437, 100.23190366735862),
                (0.01615134009865926, 100.25765583287253, 100.24150449277387)
            ],
            bulk::mcginley_dynamic_macd_line(&prices, &3_usize, &0.0, &5_usize, &0.0)
        );
    }

    #[test]
    fn test_bulk_mcginley_macd_previous() {
        let prices = vec![100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        assert_eq!(
            vec![
                (0.014602444905747802, 100.24650611226437, 100.23190366735862),
                (0.01615134009865926, 100.25765583287253, 100.24150449277387)
            ],
            bulk::mcginley_dynamic_macd_line(&prices, &3_usize, &100.21, &5_usize, &100.21)
        );
    }

    #[test]
    #[should_panic]
    fn test_bulk_mcginley_macd_panic_long_period() {
        let prices = vec![100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        bulk::mcginley_dynamic_macd_line(&prices, &3_usize, &100.21, &50_usize, &100.21);
    }

    #[test]
    #[should_panic]
    fn test_bulk_mcginley_macd_panic_no_prices() {
        let prices = Vec::new();
        bulk::mcginley_dynamic_macd_line(&prices, &3_usize, &100.21, &5_usize, &100.21);
    }
}
