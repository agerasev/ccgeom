use vecmat::{Vector, transform::{Rotation3, Shift}, distr::{Normal, Uniform}};
use approx::assert_abs_diff_eq;
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use crate::{Geometry3, Euclidean3 as Eu3, Map};
use super::Homogenous3;

const SAMPLE_ATTEMPTS: usize = 256;
const EPS: f64 = 1e-14;

#[test]
fn distance_invariance() {
    let mut rng = XorShiftRng::seed_from_u64(0xCCE);
    for _ in 0..SAMPLE_ATTEMPTS {
        let a: Vector<f64, 3> = rng.sample(&Normal);
        let b: Vector<f64, 3> = rng.sample(&Normal);

        let m = Homogenous3::new(
            rng.sample::<Shift<f64, 3>, _>(&Normal),
            rng.sample::<Rotation3<f64>, _>(&Uniform),
        );

        let dist_before = Eu3::distance(a, b);
        let dist_after = Eu3::distance(
            m.apply_pos(a),
            m.apply_pos(b),
        );

        assert_abs_diff_eq!(dist_before, dist_after, epsilon = EPS);
    }
}

#[test]
fn look_at_the_point() {
    let mut rng = XorShiftRng::seed_from_u64(0xCCE);
    for _ in 0..SAMPLE_ATTEMPTS {
        let q: Vector<f64, 3> = rng.sample(&Normal);
        let p: Vector<f64, 3> = Eu3::look_at_pos(q).apply_pos(q);

        assert_abs_diff_eq!(p, Vector::from([0.0, 0.0, -q.length()]), epsilon = EPS);
    }
}

#[test]
fn move_at_the_point() {
    let mut rng = XorShiftRng::seed_from_u64(0xCCE);
    for _ in 0..SAMPLE_ATTEMPTS {
        let p: Vector<f64, 3> = rng.sample(&Normal);
        let q: Vector<f64, 3> = rng.sample(&Normal);

        let a: Homogenous3<f64> = Eu3::move_at_pos(p);
        assert_abs_diff_eq!(a.apply_pos(p), Eu3::origin(), epsilon = EPS);

        let b: Homogenous3<f64> = Eu3::move_at_pos(q).inv().chain(a);
        assert_abs_diff_eq!(b.apply_pos(p), q, epsilon = EPS);
    }
}
