use num_traits::{Float, FloatConst, NumCast};

pub trait Scalar: Float + NumCast + FloatConst {}

impl<T> Scalar for T where T: Float + NumCast + FloatConst {}
