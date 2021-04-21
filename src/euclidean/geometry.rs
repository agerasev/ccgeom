use std::marker::PhantomData;
use num_traits::{Zero};
use vecmat::{Vector, transform::{Transform, Shift, Rotation3}};
use crate::{Geometry3, Scalar};
use super::Homogenous3;

enum EmptyEnum {}
#[allow(dead_code)]
pub struct Euclidean3<T: Scalar = f64> {
    phantom: PhantomData<T>,
    empty_enum: EmptyEnum,
}

impl<T: Scalar> Euclidean3<T> {
    fn shift(pos: <Self as Geometry3<T>>::Pos) -> <Self as Geometry3<T>>::Map {
        Homogenous3::new(pos.into(), Rotation3::identity())
    }
    fn rotate(axis: <Self as Geometry3<T>>::Dir, phi: T) -> <Self as Geometry3<T>>::Map {
        Homogenous3::new(Shift::identity(), Rotation3::new(axis, phi))
    }
}

impl<T: Scalar> Geometry3<T> for Euclidean3<T> {
    type Pos = Vector<T, 3>;
    type Dir = Vector<T, 3>;
    type Map = Homogenous3<T>;

    fn origin() -> Self::Pos {
        Self::Pos::zero()
    }

    fn dir_to_local(_pos: Self::Pos, dir: Self::Dir) -> Self::Dir {
        dir
    }
    fn dir_from_local(_pos: Self::Pos, dir: Self::Dir) -> Self::Dir {
        dir
    }

    fn length(a: Self::Pos) -> T {
        a.length()
    }
    fn distance(a: Self::Pos, b: Self::Pos) -> T {
        (a - b).length()
    }

    fn dir_when_moved_at_pos(_src_pos: Self::Pos, src_dir: Self::Dir, _dst_pos: Self::Pos) -> Self::Dir {
        src_dir
    }

    fn shift_x(dist: T) -> Self::Map {
        Self::shift(Vector::from([dist, T::zero(), T::zero()]))
    }
    fn shift_y(dist: T) -> Self::Map {
        Self::shift(Vector::from([T::zero(), dist, T::zero()]))
    }
    fn shift_z(dist: T) -> Self::Map {
        Self::shift(Vector::from([T::zero(), T::zero(), dist]))
    }

    fn rotate_x(angle: T) -> Self::Map {
        Self::rotate(Vector::from([T::one(), T::zero(), T::zero()]), angle)
    }
    fn rotate_y(angle: T) -> Self::Map {
        Self::rotate(Vector::from([T::zero(), T::one(), T::zero()]), angle)
    }
    fn rotate_z(angle: T) -> Self::Map {
        Self::rotate(Vector::from([T::zero(), T::zero(), T::one()]), angle)
    }

    fn look_at_pos(pos: Self::Pos) -> Self::Map {
        Self::look_at_dir(pos.normalize())
    }
    fn look_at_dir(dir: Self::Dir) -> Self::Map {
        Homogenous3::new(Shift::identity(), Rotation3::look_at_any(dir))
    }

    fn move_at_pos(pos: Self::Pos) -> Self::Map {
        Self::shift(pos)
    }
    fn move_at_dir(dir: Self::Dir, dist: T) -> Self::Map {
        Self::move_at_pos(dir * dist)
    }
}
