use anyhow::Result;
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

pub fn sqrt_with_mod(n: BigInt, p: BigInt) -> Result<BigInt> {
    if p < BigInt::from(3) {
        return Err(anyhow::anyhow!("moduler p {} must be larger than 3", p));
    }

    if !is_residue(&n, &p) {
        return Err(anyhow::anyhow!("n {} is not residue mod p {}", n, p));
    }

    if p.clone() % BigInt::from(4) == BigInt::from(3) {
        let x: BigInt = n.modpow(&((p.clone() + BigInt::from(1)) / BigInt::from(4)), &p);
        debug_assert_eq!(x.modpow(&BigInt::from(2), &p), n);
        return Ok(x);
    }

    let mut s = BigInt::from(0);
    let mut q = n.clone();
    while q.clone() % 2 == BigInt::from(0) {
        s += 1;
        q /= 2;
    }
    let mut m = s.clone();
    let mut z = BigInt::from(2);
    while !is_residue(&z, &p) {
        z += 1;
    }
    let mut c = z.modpow(&q, &p);
    let mut t = n.modpow(&q, &p);
    let mut r = n.modpow(&((q + 1) / 2), &p);

    // mを減らす
    if m > BigInt::from(32) {
        panic!("m {} > 32bit", m);
    }
    while m.clone() != BigInt::from(1) {
        // TODO: 2^(m - 1)がmodpowになるのが怖い
        if t.modpow(&BigInt::from(2).modpow(&(m.clone() - 1), &p), &p) == BigInt::from(1) {
            m -= 1;
            continue;
        }
        // debug文をwhile内で実行しているのでdebugビルドは遅い
        debug_assert_eq!(
            t.modpow(&BigInt::from(2).modpow(&(m.clone() - 1), &p), &p),
            p.clone() - 1,
        );

        debug_assert_eq!(
            c.modpow(&BigInt::from(2).modpow(&(m.clone() - 1), &p), &p),
            p.clone() - 1,
        );

        // find i
        let mut i = BigInt::from(1);
        while t.modpow(&BigInt::from(2).modpow(&i, &p), &p) != BigInt::from(1) {
            i += 1;
        }
        debug_assert!(i < m);
        let b = c.modpow(
            &BigInt::from(2).modpow(&(m.clone() - i.clone() - 1), &p),
            &p,
        );

        m = i;
        c = b.modpow(&BigInt::from(2), &p);
        t *= b.modpow(&BigInt::from(2), &p);
        r *= b;
    }
    return Ok(r);
}

/// is there sqrt(x)?
pub fn is_residue(x: &BigInt, p: &BigInt) -> bool {
    x.modpow(&((p - BigInt::from(1)) / BigInt::from(2)), &p) == BigInt::from(1)
}


#[cfg(test)]
mod tests {
    use num_bigint::BigInt;
    use quickcheck_macros::quickcheck;

    use super::*;
    use crate::consts::M;
    #[quickcheck]
    fn test_inverse(num: usize) -> bool {
        let num = BigInt::from(num) % &*M;
        if num == BigInt::from(0) {
            return true;
        }
        (num.clone() * inverse(num.clone(), M.clone())) % M.clone() == BigInt::from(1)
    }

    #[test]
    fn test_is_residue() {
        assert!(is_residue(&BigInt::from(5), &M));
    }

    #[quickcheck]
    fn test_sqrt_with_mod(a: usize) -> bool {
        let mut a = BigInt::from(a);
        a %= &*M;
        if a < BigInt::default() {
            a += &*M;
        }
        if !is_residue(&a, &M) {
            return true;
        }
        let r = sqrt_with_mod(a.clone(), M.clone()).unwrap();
        r.pow(2) % &*M == a
    }

    #[test]
    fn test_sample_sqrt_with_mod() {

        let a = 5;
        let mut a = BigInt::from(a);
        a %= &*M;
        if a < BigInt::default() {
            a += &*M;
        }
        let r = sqrt_with_mod(a.clone(), M.clone()).unwrap();
        dbg!(&r, r.pow(2) % &*M, &a);
        assert_eq!(r.pow(2) % &*M, a);
    }
}
