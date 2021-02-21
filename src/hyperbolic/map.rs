use vecmat::{Complex, Quaternion, complex::{Moebius}};
use crate::{geometry::{Scalar, Map}};

impl<T: Scalar> Map<Quaternion<T>, Quaternion<T>> for Moebius<Complex<T>> {
    fn identity() -> Self {
        Moebius::identity()
    }

    fn apply_pos(&self, pos: Quaternion<T>) -> Quaternion<T> {
        Moebius::apply(&self, pos)
    }
    fn apply_dir(&self, pos: Quaternion<T>, dir: Quaternion<T>) -> Quaternion<T> {
        Moebius::deriv_dir(self, pos, dir).normalize()
    }

    fn chain(self, other: Self) -> Self {
        Moebius::chain(self, other)
    }

    fn inv(self) -> Self {
        Moebius::inv(self)
    }
}
