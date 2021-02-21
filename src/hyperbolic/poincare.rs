use rand::{distributions::Distribution, Rng};
use vecmat::{complex::{Complex, Quaternion}, distr::Normal};
use crate::geometry::Scalar;

/// Poincare half-space model.
pub trait Poincare3<T: Scalar> {
    fn hx(&self) -> T;
    fn hy(&self) -> T;
    fn hz(&self) -> T;
    fn hxy(&self) -> Complex<T>;
}

impl<T: Scalar> Poincare3<T> for Quaternion<T> {
    fn hx(&self) -> T {
        self.w()
    }
    fn hy(&self) -> T {
        self.x()
    }
    fn hz(&self) -> T {
        self.y()
    }
    fn hxy(&self) -> Complex<T> {
        Complex::new(self.w(), self.x())
    } 
}

pub struct Poincare3Normal;

impl<T: Scalar> Distribution<Quaternion<T>> for Poincare3Normal
where
    Normal: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Quaternion<T> {
        Quaternion::new(
            rng.sample(&Normal),
            rng.sample(&Normal),
            rng.sample(&Normal).exp(),
            T::zero(),
        )
    }
}