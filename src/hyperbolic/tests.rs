use vecmat::{complex::{Complex, Quaternion, Moebius}, distr::Normal};
use approx::assert_abs_diff_eq;
use rand::{Rng, SeedableRng, distributions::Uniform as Uniform};
use rand_xorshift::XorShiftRng;
use crate::{Geometry, Geometry3, Map, hyperbolic::{Hyperbolic3 as Hy3, Poincare3, Poincare3Normal}};
use std::f64::consts::PI;
use num_traits::{Zero};

const SAMPLE_ATTEMPTS: usize = 256;
const EPS: f64 = 1e-14;

#[test]
fn distance_invariance() {
    let elems: Vec<Box<dyn Fn(&mut XorShiftRng) -> _>> = vec![
        Box::new(|rng| Hy3::rotate_y(rng.sample(&Uniform::from(0.0..(2.0 * PI))))),
        Box::new(|rng| Hy3::rotate_z(rng.sample(&Uniform::from(0.0..(2.0 * PI))))),
        Box::new(|rng| Hy3::shift_z(rng.sample(&Normal))),
    ];

    let mut rng = XorShiftRng::seed_from_u64(0xCCA);
    for _ in 0..SAMPLE_ATTEMPTS {
        let a: Quaternion<f64> = rng.sample(&Poincare3Normal);
        let b: Quaternion<f64> = rng.sample(&Poincare3Normal);

        let mut m = Moebius::identity();
        let uniform = Uniform::from(0..3);
        for _ in 0..8 {
            m = m.chain(elems[rng.sample(uniform)](&mut rng));
        }

        let dist_before = Hy3::distance(a, b);
        let dist_after = Hy3::distance(m.apply_pos(a), m.apply_pos(b));

        assert_abs_diff_eq!(dist_before, dist_after, epsilon = EPS);
    }
}

#[test]
fn look_at_the_point() {
    let mut rng = XorShiftRng::seed_from_u64(0xCCA);
    for _ in 0..SAMPLE_ATTEMPTS {
        let q: Quaternion<f64> = rng.sample(&Poincare3Normal);

        let p: Quaternion<f64> = Hy3::look_at_pos(q).inv().apply_pos(q);
        assert_abs_diff_eq!(p.hxy(), Complex::zero(), epsilon = EPS.sqrt());
    }
}

#[test]
fn rotation_of_derivative() {
    let mut rng = XorShiftRng::seed_from_u64(0xCCA);
    for _ in 0..SAMPLE_ATTEMPTS {
        let q: Quaternion<f64> = rng.sample(&Poincare3Normal).normalize();
        let phi = -q.hy().atan2(q.hx());
        let theta = -q.hxy().norm().atan2(q.hz());

        let c = Hy3::rotate_y(theta).chain(Hy3::rotate_z(phi));

        assert_abs_diff_eq!(c.apply_dir(Quaternion::j(), q), Quaternion::j(), epsilon = EPS);
    }
}

#[test]
fn move_at_the_point() {
    let mut rng = XorShiftRng::seed_from_u64(0xCCA);
    for _ in 0..SAMPLE_ATTEMPTS {
        let p: Quaternion<f64> = rng.sample(&Poincare3Normal);
        let q: Quaternion<f64> = rng.sample(&Poincare3Normal);

        let a: Moebius<Complex<f64>> = Hy3::move_at_pos(p).inv();
        assert_abs_diff_eq!(a.apply_pos(p), Hy3::origin(), epsilon = EPS);

        let b: Moebius<Complex<f64>> = Hy3::move_at_pos(q).chain(a);
        assert_abs_diff_eq!(b.apply_pos(p), q, epsilon = EPS.sqrt());
    }
}

// TODO: Implement when eigen decomposition is ready.
/*
TEST_F(HyperbolicTest, rotation_interpolation_at_small_angles) {
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        Moebius m = hy_yrotate(1e-3*PI*rng.uniform());
        int q = (int)floor(8*rng.uniform()) + 2;

        Moebius l = mo_identity();
        for (int i = 0; i < q; ++i) {
            l = mo_chain(l, m);
        }
        Moebius o = mo_pow(l, R1/q);
        ASSERT_EQ(c22_det(o), approx(C1));
        ASSERT_EQ(o, approx(m).epsilon(pow(EPS, (real)2/3)));
    }
}

TEST_F(HyperbolicTest, interpolation) {
    Moebius a = hy_xshift(R1);
    Moebius b = hy_yshift(R1);
    quat aq = hy_apply_pos(a, QJ), bq = hy_apply_pos(b, QJ);
    real d = hy_distance(aq, bq);
    int n = 10;
    for (int i = 0; i < n; ++i) {
        real t = real(i)/(n - 1);
        Moebius c = mo_chain(a, mo_pow(mo_chain(mo_inverse(a), b), t));
        quat cq = hy_apply_pos(c, QJ);
        ASSERT_EQ(hy_distance(aq, cq)/d, approx(t).epsilon((real)0.01f));
    }
}
*/
