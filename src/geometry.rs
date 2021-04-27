use crate::{Scalar, Map, euclidean::Euclidean3};

pub trait Geometry3<T: Scalar = f64>: 'static {
    type Pos;
    type Dir;
    type Map: Map<Self::Pos, Self::Dir>;

    /// Point of origin.
    fn origin() -> Self::Pos;
    /// Default direction at the point of origin.
    fn default_dir() -> Self::Dir;

    fn dir_to_local(pos: Self::Pos, dir: Self::Dir) -> <Euclidean3<T> as Geometry3<T>>::Dir;
    fn dir_from_local(pos: Self::Pos, dir: <Euclidean3<T> as Geometry3<T>>::Dir) -> Self::Dir;

    fn length(a: Self::Pos) -> T;
    fn distance(a: Self::Pos, b: Self::Pos) -> T;

    /// Returns the direction of the line at point `dst_pos`
    /// when we know that the line at the point `src_pos` has direction of `src_dir`.
    fn dir_when_moved_at_pos(src_pos: Self::Pos, src_dir: Self::Dir, dst_pos: Self::Pos) -> Self::Dir;

    // TODO: Replace `shift_(x,y,z)` with single function.
    //fn shift(pos: Self::Pos) -> Self::Map;
    fn shift_x(dist: T) -> Self::Map;
    fn shift_y(dist: T) -> Self::Map;
    fn shift_z(dist: T) -> Self::Map;

    // TODO: Replace `rotate_(x,y,z)` with single function.
    //fn rotate(axis: Self::Dir, phi: T) -> Self::Map;
    fn rotate_x(angle: T) -> Self::Map;
    fn rotate_y(angle: T) -> Self::Map;
    fn rotate_z(angle: T) -> Self::Map;

    /// Rotatates `pos` around the origin to make the `default_dir` point to it.
    fn look_at_pos(pos: Self::Pos) -> Self::Map;
    /// Turns `dir` into the `default_dir`.
    fn look_at_dir(dir: Self::Dir) -> Self::Map;

    // FIXME: Preserve rotation around connection line.
    /// Returns maping that translates `pos` to the origin preserving orientation
    /// relatively to the line that connects the origin to `pos`.
    fn move_at_pos(pos: Self::Pos) -> Self::Map;
    fn move_at_dir(dir: Self::Dir, dist: T) -> Self::Map;
}
