use num_traits::{Float, FloatConst, NumCast};

pub trait Scalar: Float + NumCast + FloatConst + 'static {}

impl<T> Scalar for T where T: Float + NumCast + FloatConst + 'static {}
