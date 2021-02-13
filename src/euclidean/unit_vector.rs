use num_traits::Float;
use vecmat::vector::Vector;

pub struct UnitVector<T: Float, const N: usize> {
    vec: Vector<T, N>,
}

impl<T: Float, const N: usize> UnitVector<T, N> {
    /// # Safety
    /// `vec` should be already normalized.
    pub unsafe fn from_vec_unchecked(vec: Vector<T, N>) -> Self {
        UnitVector { vec }
    }
    pub fn from_vec(vec: Vector<T, N>) -> Self {
        Self { vec: vec.normalize() }
    }
    pub fn into_vec(self) -> Vector<T, N> {
        self.vec
    }
    pub fn as_vec(&self) -> &Vector<T, N> {
        &self.vec
    }
}

impl<T: Float, const N: usize> From<Vector<T, N>> for UnitVector<T, N> {
    fn from(vec: Vector<T, N>) -> Self {
        Self::from_vec(vec)
    }
}
impl<T: Float, const N: usize> From<UnitVector<T, N>> for Vector<T, N> {
    fn from(uvec: UnitVector<T, N>) -> Self {
        uvec.into_vec()
    }
}
impl<'a, T: Float, const N: usize> From<&'a UnitVector<T, N>> for &'a Vector<T, N> {
    fn from(uvec: &'a UnitVector<T, N>) -> Self {
        uvec.as_vec()
    }
}
