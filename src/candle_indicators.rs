//! # Candle indicators
//!
//! Candle indicators are indicators that are used with candle charts.

/// `single` module holds functions that return a singular values
pub mod single {
    use crate::basic_indicators::single::{median, mode};
    use crate::moving_average::single::{moving_average, mcginley_dynamic};
    use crate::ConstantModelType;
    use crate::MovingAverageType;
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
    /// * `prices` - An `f64` slice of prices
    /// * `constant_model_type` - A variant of the `ConstantModelType` enum.
    /// * `difference` - The percent difference or distance that the bands will be from the moving
    /// constant
    ///
    /// # Examples
    ///
    /// ```
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
        constant_model_type: &crate::ConstantModelType,
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
    /// * `prices` - An `f64` slice of prices
    /// * `difference` - The percent difference or distance that the bands will be from the moving
    /// constant
    /// * `previous_mcginley_dynamic` - Previous value for the McGinley dynamic. 0.0 is no
    /// previous.
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
        let mcginley_dynamic = mcginley_dynamic(&last_price, previous_mcginley_dynamic, &prices.len());
        let upper_envelope = &mcginley_dynamic * (1.0 + (difference / 100.0));
        let lower_envelope = &mcginley_dynamic * (1.0 - (difference / 100.0));
        return (lower_envelope, mcginley_dynamic, upper_envelope);
    }

}

/// `bulk` module holds functions that return multiple valus for `momentum_indicators`
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
    /// * `prices` - An `f64` slice of prices
    /// * `constant_model_type` - A variant of the `ConstantModelType` enum.
    /// * `difference` - The percent difference or distance that the bands will be from the moving
    /// constant
    /// * `period` - Period over which to calculate the moving constant envelopes 
    ///
    /// # Examples
    ///
    /// ```
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
        period: &usize
    ) -> Vec<(f64, f64, f64)> {
        let length = prices.len();
        if period > &length {
            panic!("Period ({}) cannot be greater than length of prices ({})", period, length);
        };

        let mut moving_envelopes = Vec::new();
        let loop_max = length - period + 1;
        for i in 0..loop_max {
            moving_envelopes.push(single::moving_constant_envelopes(
                    &prices[i..i + period],
                    constant_model_type,
                    difference
            ));
        };
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
    /// * `prices` - An `f64` slice of prices
    /// * `difference` - The percent difference or distance that the bands will be from the moving
    /// constant
    /// * `previous_mcginley_dynamic` - Previous value for the McGinley dynamic. 0.0 is no
    /// previous.
    /// * `period` - Period over which to calculate the McGinley dynamic envelopes.
    /// # Examples
    ///
    /// ```
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
                    &previous_dynamic
            ));
        };
        return mcginley_envelopes;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_ma_moving_constant_envelope() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!((97.34338, 100.354, 103.36462), single::moving_constant_envelopes(&prices, &crate::ConstantModelType::SimpleMovingAverage, &3.0));
    }

    #[test]
    fn test_single_sma_moving_constant_envelope() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!((97.30730209424084, 100.31680628272251, 103.32631047120418), single::moving_constant_envelopes(&prices, &crate::ConstantModelType::SmoothedMovingAverage, &3.0));
    }

    #[test]
    fn test_single_ema_moving_constant_envelope() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!((97.28056445497631, 100.28924170616115, 103.29791895734598), single::moving_constant_envelopes(&prices, &crate::ConstantModelType::ExponentialMovingAverage, &3.0));
    }

    #[test]
    fn test_single_pma_moving_constant_envelope() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!((97.2379964584231, 100.24535717363206, 103.25271788884102), single::moving_constant_envelopes(&prices, &crate::ConstantModelType::PersonalisedMovingAverage(&5.0, &4.0), &3.0));
    }

    #[test]
    fn test_single_median_moving_constant_envelope() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!((97.36859999999999, 100.38, 103.3914), single::moving_constant_envelopes(&prices, &crate::ConstantModelType::SimpleMovingMedian, &3.0));
    }

    #[test]
    fn test_single_mode_moving_constant_envelope() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!((97.0, 100.0, 103.0), single::moving_constant_envelopes(&prices, &crate::ConstantModelType::SimpleMovingMode, &3.0));
    }

    #[test]
    #[should_panic]
    fn test_single_moving_constant_envelope_panic() {
        let prices = Vec::new();
        single::moving_constant_envelopes(&prices, &crate::ConstantModelType::SimpleMovingMode, &3.0);
    }

    #[test]
    fn test_bulk_moving_constant_envelope() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        assert_eq!(vec![(97.28056445497631, 100.28924170616115, 103.29791895734598), (97.28364454976304, 100.29241706161139, 103.30118957345974), (97.26737061611377, 100.27563981042657, 103.28390900473937)], bulk::moving_constant_envelopes(&prices, &crate::ConstantModelType::ExponentialMovingAverage, &3.0, &5_usize));
    }

    #[test]
    #[should_panic]
    fn test_bulk_moving_constant_envelope_panic() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        bulk::moving_constant_envelopes(&prices, &crate::ConstantModelType::ExponentialMovingAverage, &3.0, &50_usize);
    }

    #[test]
    fn test_single_mcginley_envelope_no_previous() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!((97.2037, 100.21, 103.21629999999999), single::mcginley_dynamic_envelopes(&prices, &3.0, &0.0));
    }

    #[test]
    fn test_single_mcginley_envelope_previous() {
        let prices = vec![100.53, 100.38, 100.19, 100.21, 100.32];
        assert_eq!((97.22494655733786, 100.23190366735862, 103.23886077737939), single::mcginley_dynamic_envelopes(&prices, &3.0, &100.21));
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
        assert_eq!(vec![(97.2037, 100.21, 103.21629999999999), (97.22494655733786, 100.23190366735862, 103.23886077737939), (97.23425935799065, 100.24150449277387, 103.24874962755709)], bulk::mcginley_dynamic_envelopes(&prices, &3.0, &0.0, &5_usize));
    }

    #[test]
    fn test_bulk_mcginley_envelope_previous() {
        let prices = vec![100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        assert_eq!(vec![(97.22494655733786, 100.23190366735862, 103.23886077737939), (97.23425935799065, 100.24150449277387, 103.24874962755709 )], bulk::mcginley_dynamic_envelopes(&prices, &3.0, &100.21, &5_usize));
    }

    #[test]
    #[should_panic]
    fn test_bulk_mcginley_envelope_panic() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        bulk::mcginley_dynamic_envelopes(&prices, &3.0, &0.0, &50_usize);
    }
}
