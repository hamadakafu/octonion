use std::str::FromStr;

use anyhow::Result;
use num_bigint::BigInt;
use num_bigint::RandBigInt;

use crate::types::Octonion;

/// find num, num^-1 mod m
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

/// find r, r^2 === n mod p
pub fn sqrt_with_mod(n: BigInt, p: BigInt) -> Result<BigInt> {
    if p < BigInt::from(3) {
        return Err(anyhow::anyhow!("moduler p {} must be larger than 3", p));
    }

    if &p % 2 == BigInt::from(0) {
        return Err(anyhow::anyhow!("moduler p {} must be odd! maybe...", p));
    }

    if !is_residue(&n, &p) {
        return Err(anyhow::anyhow!("n {} is not residue mod p {}", n, p));
    }

    if p.clone() % BigInt::from(4) == BigInt::from(3) {
        let x: BigInt = n.modpow(&((&p + 1) / 4), &p);
        debug_assert_eq!(x.modpow(&BigInt::from(2), &p), n);
        return Ok(x);
    }

    let mut s = BigInt::from(0);
    let mut q: BigInt = &p - 1;
    while q.clone() % 2 == BigInt::from(0) {
        s += 1;
        q /= 2;
    }
    let mut z = BigInt::from(2);
    while is_residue(&z, &p) {
        z += 1;
    }

    // initial values
    let mut m = s.clone();
    let mut c = z.modpow(&q, &p);
    let mut t = n.modpow(&q, &p);
    let mut r = n.modpow(&((q + 1) / 2), &p);

    // decrease m
    loop {
        if t == BigInt::from(0) {
            break Ok(BigInt::from(0));
        }
        if t == BigInt::from(1) {
            break Ok(r);
        }

        // find least i, 0 < i < m
        let mut i = BigInt::from(1);
        // p-1 = q*2^s
        // m less than s which is initial value, then satisfy
        // 2^(m-1) < p
        if t.modpow(&BigInt::from(2).modpow(&(&m - 1), &p), &p) == BigInt::from(1) {
            i = &m - 1;
        } else {
            while t.modpow(&BigInt::from(2).modpow(&i, &p), &p) != BigInt::from(1) {
                i += 1;
                debug_assert!(i < m);
            }
        }

        let b = c.modpow(&BigInt::from(2).modpow(&(&m - &i - 1), &p), &p);
        m = i;
        c = b.modpow(&BigInt::from(2), &p);
        t *= &c;
        t %= &p;
        r *= &b;
        r %= &p;

        debug_assert_eq!(r.modpow(&BigInt::from(2), &p), (&n * &t) % &p,);
        debug_assert_eq!(
            t.modpow(&BigInt::from(2).modpow(&(&m - 1), &p), &p),
            BigInt::from(1),
        );
        debug_assert_eq!(c.modpow(&BigInt::from(2).modpow(&(&m - 1), &p), &p), &p - 1,);
    }
}

/// is there sqrt(x)?
pub fn is_residue(x: &BigInt, p: &BigInt) -> bool {
    x.modpow(&((p - BigInt::from(1)) / BigInt::from(2)), &p) == BigInt::from(1)
}

pub fn gen_rand_octonion_which_has_inv<const MOD: &'static str>() -> Octonion<MOD> {
    let m = BigInt::from_str(MOD).unwrap();

    let mut rng = rand::thread_rng();
    let a0: BigInt = rng.gen_bigint_range(&BigInt::from(0), &m);
    let a1: BigInt = rng.gen_bigint_range(&BigInt::from(0), &m);
    let a2: BigInt = rng.gen_bigint_range(&BigInt::from(0), &m);
    let a3: BigInt = rng.gen_bigint_range(&BigInt::from(0), &m);
    let a4: BigInt = rng.gen_bigint_range(&BigInt::from(0), &m);
    let a5: BigInt = rng.gen_bigint_range(&BigInt::from(0), &m);
    let a6: BigInt = rng.gen_bigint_range(&BigInt::from(0), &m);
    let mut a7: BigInt = rng.gen_bigint_range(&BigInt::from(0), &m);

    let a = loop {
        let a = Octonion::new_with_bigint(
            a0.clone(),
            a1.clone(),
            a2.clone(),
            a3.clone(),
            a4.clone(),
            a5.clone(),
            a6.clone(),
            a7.clone(),
        );
        if a.has_inv() {
            break a;
        }
        a7 += 1;
        a7 %= &m;
    };
    return a;
}

#[cfg(test)]
mod tests {
    use num_bigint::BigInt;
    use quickcheck_macros::quickcheck;

    use super::*;
    use crate::consts::M521_STR;
    #[quickcheck]
    fn test_inverse(num: usize) -> bool {
        let m = BigInt::from_str(M521_STR).unwrap();

        let num = BigInt::from(num) % &m;
        if num == BigInt::from(0) {
            return true;
        }
        (num.clone() * inverse(num.clone(), m.clone())) % m.clone() == BigInt::from(1)
    }

    // #[test]
    // fn test_is_residue() {
    //     assert!(is_residue(&BigInt::from(5), &M));
    // }

    #[quickcheck]
    fn test_sqrt_with_mod(a: usize) -> bool {
        let m = BigInt::from_str(M521_STR).unwrap();

        let mut a = BigInt::from(a);
        a %= &m;
        if a < BigInt::default() {
            a += &m;
        }
        if !is_residue(&a, &m) {
            return true;
        }
        if let Ok(r) = sqrt_with_mod(a.clone(), m.clone()) {
            if r.modpow(&BigInt::from(2), &m) != a {
                println!("{} * {} !== {} mod {}", r, r, a, m.clone());
            }
            r.modpow(&BigInt::from(2), &m) == a
        } else {
            true
        }
    }

    // #[test]
    // fn test_sample_sqrt_with_mod() {
    //     let a = 4;
    //     let mut a = BigInt::from(a);
    //     a %= &*M;
    //     if a < BigInt::default() {
    //         a += &*M;
    //     }
    //     let r = sqrt_with_mod(a.clone(), M.clone()).unwrap();
    //     dbg!(&r, r.pow(2) % &*M, &a);
    //     assert_eq!(r.pow(2) % &*M, a);
    // }

    #[quickcheck]
    fn test_gen_rand_octonion_which_has_inv(_: usize) -> bool {
        gen_rand_octonion_which_has_inv::<M521_STR>().has_inv()
    }
}
