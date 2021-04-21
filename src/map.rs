use vecmat::{Transform, traits::Normalize};

pub trait Map<P, D> {
    fn identity() -> Self;

    fn apply_pos(&self, pos: P) -> P;
    fn apply_dir(&self, pos: P, dir: D) -> D;
    
    fn chain(self, other: Self) -> Self;
    
    fn inv(self) -> Self;
}

impl<A, T> Map<T, T> for A where A: Transform<T>, T: Normalize {
    fn identity() -> Self {
        <A as Transform<T>>::identity()
    }

    fn apply_pos(&self, pos: T) -> T {
        <A as Transform<T>>::apply(self, pos)
    }
    fn apply_dir(&self, pos: T, dir: T) -> T {
        <A as Transform<T>>::deriv(self, pos, dir).normalize()
    }

    fn chain(self, other: Self) -> Self {
        <A as Transform<T>>::chain(self, other)
    }

    fn inv(self) -> Self {
        <A as Transform<T>>::inv(self)
    }
}
