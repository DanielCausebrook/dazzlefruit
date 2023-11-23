use std::f64::consts::PI;
use num_traits::{Float, FromPrimitive};

///
/// Computes a skewed sin wave.
///
/// - `skew`: Amount of skew, in range `[-1.0, 1.0]`. `-1.0` yields a negative sawtooth wave,
/// `1.0` yields a positive sawtooth wave.
/// - `period`: Period of the wave function.
/// - `x`: Input to the wave function.
///
/// From https://math.stackexchange.com/a/2430837. Adjusted the 1/t parameter to correctly fit the
/// wave between -1 and 1.
///
pub fn skew_sin<T: Float + FromPrimitive>(skew: T, period: T, x: T) -> T {
    let x_rad = (T::from_f64(2.0 * PI).unwrap() * x) / period;
    if skew.is_zero() {
        return x_rad.sin()
    }
    let t = -skew;

    let function_maximum = (t / (T::one() - t.powi(2)).sqrt()).atan();
    ((t * x_rad.sin()) / (T::one() - t * x_rad.cos())).atan() / function_maximum
}

///
/// Computes a squared sin wave.
///
/// - `smoothness`: How smooth the wave should be. `0.0` is a square wave. In range `[0.0, 1.0]`,
/// approximately this proportion of the wave will be rounded. Higher values get closer to a sin
/// wave - at `10.0`, the output is within `0.01` of `sin(x)`.
/// - `period`: Period of the wave function.
/// - `x`: Input to the wave function.
///
pub fn square_sin<T: Float + FromPrimitive>(smoothness: T, period: T, x: T) -> T {
    let a = T::from_f64(2.5).unwrap() / smoothness;
    let x_rad = (T::from_f64(2.0 * PI).unwrap() * x) / period;

    let function_maximum = a.tanh();
    (a * x_rad.sin()).tanh() / function_maximum
}

///
/// Computes a triangled sin wave.
///
/// - `triangle_amount`: How smooth the wave should be. `0.0` is a square wave. In range `[0.0, 1.0]`,
/// approximately this proportion of the wave will be rounded. Higher values get closer to a sin
/// wave - at `10.0`, the output is within `0.01` of `sin(x)`.
/// - `period`: Period of the wave function.
/// - `x`: Input to the wave function.
///
pub fn triangle_sin<T: Float + FromPrimitive>(smoothness: T, period: T, x: T) -> T {
    if smoothness == T::zero() {
        let x = x + period/T::from_f64(4.0).unwrap();
        return T::from_f64(4.0).unwrap() * (x/period - (x/period + T::from_f64(0.5).unwrap()).floor()).abs() - T::one();
    }
    let x_rad = (T::from_f64(2.0 * PI).unwrap() * x) / period;

    let t = (T::from_f64(2.0).unwrap() / smoothness).tanh();

    let function_maximum = t.asin();
    (t * x_rad.sin()).asin() / function_maximum
}