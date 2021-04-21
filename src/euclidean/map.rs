use vecmat::{Vector, transform::{Rotation3, Shift, Chain}};

pub type Homogenous3<T> = Chain<Shift<T, 3>, Rotation3<T>, Vector<T, 3>>;
