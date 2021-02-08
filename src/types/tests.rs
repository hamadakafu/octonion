// use quickcheck_macros::quickcheck;
use quickcheck_macros::quickcheck;

use super::*;

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
