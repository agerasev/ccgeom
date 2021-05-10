use std::marker::PhantomData;
use num_traits::{Zero, One};
use vecmat::complex::{Complex, Quaternion, Moebius};
use crate::{Geometry, Geometry3, Scalar, euclidean::Euclidean3 as Eu3, hyperbolic::Poincare3};

enum EmptyEnum {}
#[allow(dead_code)]
pub struct Hyperbolic3<T: Scalar = f64> {
    phantom: PhantomData<T>,
    empty_enum: EmptyEnum,
}

impl<T: Scalar> Hyperbolic3<T> {
    /// Moves to the specified position at the horosphere.
    pub fn horosphere(pos: Complex<T>) -> Moebius<Complex<T>> {
        Moebius::new(
            Complex::one(),
            pos,
            Complex::zero(),
            Complex::one(),
        )
    }
}

impl<T: Scalar> Geometry<T> for Hyperbolic3<T> {
    type Pos = Quaternion<T>;
    type Dir = Quaternion<T>;
    type Map = Moebius<Complex<T>>;

    fn origin() -> Self::Pos {
        Quaternion::j()
    }
    fn default_dir() -> Self::Dir {
        Quaternion::j()
    }

    fn length(a: Self::Pos) -> T {
        Self::distance(a, Self::origin())
    }
    fn distance(a: Self::Pos, b: Self::Pos) -> T {
        let x = T::one() + (a - b).norm_sqr() / (T::from(2).unwrap() * a.hz() * b.hz());
        (x + (x*x - T::one()).sqrt()).ln()
    }
}

impl<T: Scalar> Geometry3<T> for Hyperbolic3<T> {
    fn dir_to_local(_pos: Self::Pos, dir: Self::Dir) -> <Eu3<T> as Geometry<T>>::Dir {
        let (x, y, z, _) = dir.into();
        (x, y, z).into()
    }
    fn dir_from_local(_pos: Self::Pos, dir: <Eu3<T> as Geometry<T>>::Dir) -> Self::Dir {
        let (x, y, z) = dir.into();
        (x, y, z, T::zero()).into()
    }

    /// Returns the direction of the line at point `dst_pos`
    /// when we know that the line at the point `src_pos` has direction of `src_dir`.
    fn dir_when_moved_at_pos(src_pos: Self::Pos, src_dir: Self::Dir, dst_pos: Self::Pos) -> Self::Dir {
        let (p, d, h) = (src_pos, src_dir, dst_pos);
        Quaternion::new(
            h.hz() / p.hz() * d.hx(),
            h.hz() / p.hz() * d.hy(),
            d.hz() - (p.hxy() - h.hxy()).norm() / p.hz() * d.hxy().norm(),
            T::zero(),
        )
    }

    fn shift_x(dist: T) -> Self::Map {
        let l2 = dist / T::from(2).unwrap();
        let (c, s) = (l2.cosh(), l2.sinh());
        let (c1, s1) = (Complex::new(c, T::zero()), Complex::new(s, T::zero()));
        Moebius::new(c1, s1, s1, c1)
    }
    fn shift_y(dist: T) -> Self::Map {
        let l2 = dist / T::from(2).unwrap();
        let (c, s) = (l2.cosh(), l2.sinh());
        let (c1, si) = (Complex::new(c, T::zero()), Complex::new(T::zero(), s));
        Moebius::new(c1, si, -si, c1)
    }
    fn shift_z(dist: T) -> Self::Map {
        let l2 = dist / T::from(2).unwrap();
        let e = l2.exp();
        Moebius::new(
            Complex::new(e, T::zero()),
            Complex::zero(),
            Complex::zero(),
            Complex::new(T::one() / e, T::zero()),
        )
    }

    fn rotate_x(angle: T) -> Self::Map {
        let ht = angle / T::from(2).unwrap();
        let (c, s) = (ht.cos(), ht.sin());
        let (c1, si) = (Complex::new(c, T::zero()), Complex::new(T::zero(), s));
        Moebius::new(c1, si, si, c1)
    }
    fn rotate_y(angle: T) -> Self::Map {
        let ht = angle / T::from(2).unwrap();
        let (c, s) = (ht.cos(), ht.sin());
        let (c1, s1) = (Complex::new(c, T::zero()), Complex::new(s, T::zero()));
        Moebius::new(c1, -s1, s1, c1)
    }
    fn rotate_z(angle: T) -> Self::Map {
        let hp = angle / T::from(2).unwrap();
        let (c, s) = (hp.cos(), hp.sin());
        Moebius::new(
            Complex::new(c, s),
            Complex::zero(),
            Complex::zero(),
            Complex::new(c, -s),
        )
    }

    /// Rotatates point `pos` around the origin to make it lay on the z axis.
    fn look_at_pos(pos: Self::Pos) -> Self::Map {
        // The origin is at *j* (z = 1).
        let phi = -pos.hy().atan2(pos.hx());
        let theta = -(T::from(2).unwrap() * pos.hxy().norm()).atan2(pos.norm_sqr() - T::one());
        Self::rotate_y(theta).chain(Self::rotate_z(phi))
    }
    /// Turns direction `dir` to *j*.
    fn look_at_dir(dir: Self::Dir) -> Self::Map {
        // We look at the top (along the z axis).
        let phi = -dir.hy().atan2(dir.hx());
        let theta = -dir.hxy().norm().atan2(dir.hz());
        Self::rotate_y(theta).chain(Self::rotate_z(phi))
    }

    /// Translates point `pos` to the origin preserving orientation
    /// relative to the line that connects `pos` to the origin.
    fn move_at_pos(pos: Self::Pos) -> Self::Map {
        let a = Self::look_at_pos(pos);
        let b = Self::shift_z(-Self::length(pos));
        a.inv().chain(b).chain(a)
    }
    fn move_at_dir(dir: Self::Dir, dist: T) -> Self::Map {
        let a = Self::look_at_dir(dir);
        let b = Self::shift_z(-dist);
        a.inv().chain(b).chain(a)
    }
}
