use num_bigint::RandBigInt;
use quickcheck::{Arbitrary, Gen};
use quickcheck_macros::quickcheck;

use super::*;
use crate::consts::M9689;

impl Arbitrary for Octonion {
    fn arbitrary(_: &mut Gen) -> Self {
        let mut rng = rand::thread_rng();
        let a0 = rng.gen_bigint_range(&BigInt::from(0), &M9689);
        let a1 = rng.gen_bigint_range(&BigInt::from(0), &M9689);
        let a2 = rng.gen_bigint_range(&BigInt::from(0), &M9689);
        let a3 = rng.gen_bigint_range(&BigInt::from(0), &M9689);
        let a4 = rng.gen_bigint_range(&BigInt::from(0), &M9689);
        let a5 = rng.gen_bigint_range(&BigInt::from(0), &M9689);
        let a6 = rng.gen_bigint_range(&BigInt::from(0), &M9689);
        let a7 = rng.gen_bigint_range(&BigInt::from(0), &M9689);

        Octonion {
            a0,
            a1,
            a2,
            a3,
            a4,
            a5,
            a6,
            a7,
        }
    }
}

#[quickcheck]
fn test_div(a: Octonion) -> bool {
    if !a.has_inv() {
        return true;
    }
    a.clone() / a
        == Octonion::new_with_bigint(
            BigInt::from(1),
            BigInt::from(0),
            BigInt::from(0),
            BigInt::from(0),
            BigInt::from(0),
            BigInt::from(0),
            BigInt::from(0),
            BigInt::from(0),
        )
}

#[quickcheck]
fn test_inverse(a: Octonion) -> bool {
    if let Some(a_inv) = a.inverse() {
        a_inv.clone() * a.clone() == Octonion::one() && a * a_inv == Octonion::one()
    } else {
        true
    }
}

/// (aa)b = a(ab)
/// a(bb) = (ab)b
/// (ab)a = a(ba)
#[quickcheck]
fn test_alternative(a: Octonion, b: Octonion) -> bool {
    (a.clone() * a.clone()) * b.clone() == a.clone() * (a.clone() * b.clone())
        && a.clone() * (b.clone() * b.clone()) == (a.clone() * b.clone()) * b.clone()
        && (a.clone() * b.clone()) * a.clone() == a.clone() * (b.clone() * a.clone())
}

/// c(a(cb)) = ((ca)c)b
/// a(c(bc)) = ((ac)b)c
/// (ca)(bc) = (c(ab))c
/// (ca)(bc) = c((ab)c)
#[quickcheck]
fn test_moufang(a: Octonion, b: Octonion, c: Octonion) -> bool {
    c.clone() * (a.clone() * (c.clone() * b.clone()))
        == ((c.clone() * a.clone()) * c.clone()) * b.clone()
        && a.clone() * (c.clone() * (b.clone() * c.clone()))
            == ((a.clone() * c.clone()) * b.clone()) * c.clone()
        && (c.clone() * a.clone()) * (b.clone() * c.clone())
            == (c.clone() * (a.clone() * b.clone())) * c.clone()
        && (c.clone() * a.clone()) * (b.clone() * c.clone())
            == c.clone() * ((a.clone() * b.clone()) * c.clone())
}

/// theorem1 18式
/// 任意のA \in Oに対して
/// A^2 = w \cdot 1 + v A
/// を満たす定数(\in Fp)w,vが存在する
#[quickcheck]
fn test_theorem1_18(a: Octonion) -> bool {
    let w0 = -a.norm2();
    let w = &w0 * Octonion::one();
    let v = (2 * a.a0.clone()) % &*M;
    a.clone() * a.clone() == w + &v * a.clone()
}

/// a^2 =  (2a_0^2 - L, 2a_0a_1, 2a_0 a_2, \cdots, 2a_0 a_7)
#[quickcheck]
fn test_aa(a: Octonion) -> bool {
    let aa = a.clone() * a.clone();
    let l = a.norm2();
    let mut ideal_a0 = (BigInt::from(2) * a.a0.clone().pow(2) - l.clone()) % &*M;
    if ideal_a0 < BigInt::from(0) {
        ideal_a0 += &*M;
    }
    assert_eq!(ideal_a0, aa.a0);
    assert_eq!((BigInt::from(2) * a.a0.clone() * a.a1.clone()) % &*M, aa.a1);
    assert_eq!((BigInt::from(2) * a.a0.clone() * a.a2.clone()) % &*M, aa.a2);
    assert_eq!((BigInt::from(2) * a.a0.clone() * a.a3.clone()) % &*M, aa.a3);
    assert_eq!((BigInt::from(2) * a.a0.clone() * a.a4.clone()) % &*M, aa.a4);
    assert_eq!((BigInt::from(2) * a.a0.clone() * a.a5.clone()) % &*M, aa.a5);
    assert_eq!((BigInt::from(2) * a.a0.clone() * a.a6.clone()) % &*M, aa.a6);
    assert_eq!((BigInt::from(2) * a.a0.clone() * a.a7.clone()) % &*M, aa.a7);
    return true;
}

#[quickcheck]
fn test_mul_div(a: Octonion, b: Octonion) -> bool {
    if !a.has_inv() || !b.has_inv() {
        return true;
    }
    let left = ((Octonion::one() / a.clone()) * b.clone()) * a.clone();
    let right = (Octonion::one() / a.clone()) * (b.clone() * a.clone());
    if left != right {
        println!("{:#?}\n{:#?}", left, right);
        println!("a: {:#?}\nb: {:#?}", a, b);
    }
    return left == right;
}
