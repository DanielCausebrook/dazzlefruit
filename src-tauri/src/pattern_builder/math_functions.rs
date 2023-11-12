use num_traits::Float;

///
/// Computes a skewed sin wave.
///
/// `x` is the input to the sin function, in radians.
/// `skew` is the amount of skew, in range `[-1.0, 1.0]`. `-1.0` yields a negative sawtooth wave,
/// `1.0` yields a positive sawtooth wave.
///
/// From https://math.stackexchange.com/a/2430837
///
pub fn skew_sin<T: Float>(x: T, skew: T) -> T {
    let t = -skew;
    (T::one() / t).atan() * ((t * x.sin()) / (T::one() - t * x.cos()))
}
