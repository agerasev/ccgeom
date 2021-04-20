use vecmat::{Vector, transform::{Transform, Rotation3, Shift, Chain}};
use crate::geometry::{Scalar, Map};

pub type Homogenous3<T> = Chain<Shift<T, 3>, Rotation3<T>, T, 3>;

impl<T: Scalar> Map<Vector<T, 3>, Vector<T, 3>> for Homogenous3<T> {
    fn identity() -> Self {
        <Self as Transform<T, 3>>::identity()
    }

    fn apply_pos(&self, pos: Vector<T, 3>) -> Vector<T, 3> {
        <Self as Transform<T, 3>>::apply(self, pos)
    }
    fn apply_dir(&self, pos: Vector<T, 3>, dir: Vector<T, 3>) -> Vector<T, 3> {
        <Self as Transform<T, 3>>::deriv(self, pos, dir).normalize()
    }

    fn chain(self, other: Self) -> Self {
        <Self as Transform<T, 3>>::chain(self, other)
    }

    fn inv(self) -> Self {
        <Self as Transform<T, 3>>::inv(self)
    }
}
