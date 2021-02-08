use num_bigint::BigInt;
/// modを法として逆元求める
pub fn inverse(num: BigInt, m: BigInt) -> BigInt {
    let mut x0 = BigInt::from(1);
    let mut y0 = BigInt::from(0);
    let mut x1 = BigInt::from(0);
    let mut y1 = BigInt::from(1);
    let mut a = num.clone() % m.clone();
    let mut b = m.clone();
    let q;
    while b != BigInt::from(0) {
        let q = a.clone() / b.clone();
        let pre_b = b.clone();
        let pre_a = a.clone();
        a = pre_b.clone();
        b = pre_a % pre_b;

        let pre_x0 = x0;
        let pre_x1 = x1;
        x0 = pre_x1.clone();
        x1 = pre_x0 - q.clone() * pre_x1;

        let pre_y0 = y0;
        let pre_y1 = y1;
        y0 = pre_y1.clone();
        y1 = pre_y0 - q * pre_y1;
    }
    if a != BigInt::from(1) {
        dbg!(a, b, x0, x1, y0, y1);
        panic!(
            "modular inverse does not exist for num: {:?}, moduler: {:?}",
            num, m
        );
    }

    if x0 < BigInt::from(0) {
        q = x0.clone() / m.clone();
        x0 -= (q - 1) * m.clone();
    }

    return x0 % m;
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use num_bigint::BigInt;
    use quickcheck_macros::quickcheck;

    use super::inverse;
    use crate::consts::M;
    #[quickcheck]
    fn test_inverse(num: usize) -> bool {
        let num = BigInt::from(num) % &*M;
        if num == BigInt::from(0) {
            return true;
        }
        (num.clone() * inverse(num.clone(), M.clone())) % M.clone() == BigInt::from(1)
    }
}
