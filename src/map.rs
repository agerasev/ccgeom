use vecmat::{Transform, transform::Directional, traits::Normalize};

pub trait Map<P, D = P> {
    fn identity() -> Self;

    fn apply_pos(&self, pos: P) -> P;
    fn apply_dir(&self, pos: P, dir: D) -> D;
    fn apply_normal(&self, pos: P, normal: D) -> D;
    
    fn chain(self, other: Self) -> Self;
    fn inv(self) -> Self;
}

impl<A, T> Map<T, T> for A where A: Directional<T>, T: Normalize {
    fn identity() -> Self {
        <A as Transform<T>>::identity()
    }

    fn apply_pos(&self, pos: T) -> T {
        <A as Transform<T>>::apply(self, pos)
    }
    fn apply_dir(&self, pos: T, dir: T) -> T {
        <A as Directional<T>>::apply_dir(self, pos, dir)
    }
    fn apply_normal(&self, pos: T, dir: T) -> T {
        <A as Directional<T>>::apply_normal(self, pos, dir)
    }

    fn chain(self, other: Self) -> Self {
        <A as Transform<T>>::chain(self, other)
    }
    fn inv(self) -> Self {
        <A as Transform<T>>::inv(self)
    }
}
