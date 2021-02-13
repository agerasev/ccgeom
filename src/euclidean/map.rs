use num_traits::{Float};
use vecmat::{Vector, transform::{Transform, Affine}};
use crate::{geometry::{Map}, UnitVector};

impl<T: Float> Map<Vector<T, 3>, UnitVector<T, 3>> for Affine<T, 3> {
    fn identity() -> Self {
        <Self as Transform<T, 3>>::identity()
    }

    fn apply_pos(self, pos: Vector<T, 3>) -> Vector<T, 3> {
        <Self as Transform<T, 3>>::apply(self, pos)
    }
    fn apply_dir(self, pos: Vector<T, 3>, dir: UnitVector<T, 3>) -> UnitVector<T, 3> {
        <Self as Transform<T, 3>>::deriv(self, pos, dir.into_vec()).into()
    }

    fn chain(self, other: Self) -> Self {
        <Self as Transform<T, 3>>::chain(self, other)
    }

    fn inv(self) -> Self {
        <Self as Transform<T, 3>>::inv(self)
    }
}
