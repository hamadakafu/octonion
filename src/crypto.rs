use std::fmt::Display;
use std::fmt::Formatter;

use num_bigint::BigInt;
use num_bigint::RandBigInt;
use rand;

use crate::crypto::cipher_text::CipherText;
use crate::types::Octonion;
use crate::utils::gen_rand_octonion_which_has_inv;
use crate::utils::inverse;
use crate::utils::is_residue;
use crate::utils::sqrt_with_mod;

mod cipher_text;
#[cfg(test)]
mod tests;

/// octonion schema
#[derive(Debug, Clone)]
pub struct Schema {
    q: BigInt,
    g: Octonion,
    h: Octonion,
}

#[derive(Debug, Clone)]
pub struct PlainText {
    pub value: BigInt,
}

#[derive(Debug, Clone)]
pub struct MediamText {
    pub value: Octonion,
}

impl MediamText {}

impl Display for MediamText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.value)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct SecretKey {
    pub q: BigInt,
    /// Aのlength
    pub h: usize,
    /// A_i
    pub a: Vec<Octonion>,
}

impl Display for SecretKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "q: {}, h: {}, a:", self.q, self.h)?;
        for a in self.a.iter() {
            write!(f, " {}", a)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct PublicKey {
    pub q: BigInt,
    /// coefficients f: (O \times O) -> O
    /// ijk -> i x y ((e000 * x0 * y0 + ... + e077 * x7 * y7), ...)
    pub e: Vec<Vec<Vec<BigInt>>>,
}

impl Display for PublicKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "q: {}\ne:", self.q)?;
        for ie in 0..8 {
            write!(f, "{}: (", ie)?;
            for ix in 0..8 {
                write!(f, "{}: (", ix)?;
                for iy in 0..8 {
                    write!(f, " {}", self.e[ie][ix][iy])?;
                }
                write!(f, ")")?;
            }
            write!(f, ")")?;
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl PublicKey {
    /// generate public key from secret key
    pub fn new_from_sk(sk: &SecretKey) -> PublicKey {
        let enc_fn = |x: Octonion, y: Octonion| {
            let mut ans = x;

            // A_h^-1 ( ... ( A_1^-1 X ) )
            for a in &sk.a {
                ans = a.inverse().unwrap() * ans;
            }

            ans = y * ans;

            // A_1 ( ... ( A_h ans ) )
            for a in sk.a.iter().rev() {
                ans = a.clone() * ans;
            }
            return ans;
        };

        let mut e = vec![vec![vec![BigInt::from(0); 8]; 8]; 8];
        for ix in 0..8 {
            for iy in 0..8 {
                let mut x = Octonion::zero();
                let mut y = Octonion::zero();
                x[ix] = BigInt::from(1);
                y[iy] = BigInt::from(1);
                let result = enc_fn(x, y);
                for ie in 0..8 {
                    e[ie][ix][iy] = result[ie].clone();
                }
            }
        }
        return PublicKey {
            q: sk.q.clone(),
            e,
        };
    }
}

impl Schema {
    pub fn new_with_q(q: BigInt) -> Self {
        let (g, h) = Self::find_g_h(q.clone());
        // TODO: validate prime
        if q < BigInt::from(0) {
            panic!("modulus q({:?}) is less than 0", q);
        }
        return Self::new_with_q_g_h(q, g, h);
    }

    pub fn new_with_q_g_h(q: BigInt, g: Octonion, h: Octonion) -> Self {
        Self { q, g, h }
    }

    pub fn new_plaintext(&self, p: BigInt) -> PlainText {
        // TODO: validate Octonion over Fq
        PlainText { value: p }
    }

    /// generate secret key and public key
    pub fn gen_sk_pk(&self) -> (SecretKey, PublicKey) {
        // TODO: how to determin SecretKey.a.len()
        let h = 56;
        let a = {
            let mut a = Vec::with_capacity(h);
            for _ in 0..h {
                a.push(gen_rand_octonion_which_has_inv(&self.q));
            }
            a
        };
        let sk = SecretKey {
            h,
            a,
            q: self.q.clone(),
        };
        let pk = PublicKey::new_from_sk(&sk);
        return (sk, pk);
    }

    pub fn encrypt(&self, pt: PlainText, pk: &PublicKey) -> CipherText {
        let mt = self.p_to_m(pt);
        let mut e = vec![vec![BigInt::from(0); 8]; 8];
        for ie in 0..8 {
            for ix in 0..8 {
                for iy in 0..8 {
                    e[ie][ix] += &pk.e[ie][ix][iy] * &mt.value[iy];
                    e[ie][ix] %= &self.q;
                }
            }
        }
        CipherText {
            q: self.q.clone(),
            e,
        }
    }

    pub fn decrypt(&self, ct: CipherText, sk: &SecretKey) -> PlainText {
        // TODO: if rust implements #[feature(fn_traits)], impl Fn for SecretKey
        let mut mt = Octonion::zero();
        let mut x = Octonion::one();
        // A_1 ( ... (A_h 1) )
        for a in sk.a.iter().rev() {
            x = a.clone() * x;
        }

        for ie in 0..8 {
            for ix in 0..8 {
                mt[ie] += &ct.e[ie][ix] * &x[ix];
            }
            mt[ie] %= &self.q;
        }

        // A_r^-1 ( ... (A_1^-1 pt) )
        for a in sk.a.iter() {
            mt = a.inverse().unwrap() * mt;
        }

        self.m_to_p(&MediamText { value: mt })
    }

    /// plaintext -> mediamtext
    fn p_to_m(&self, p: PlainText) -> MediamText {
        let mut rng = rand::thread_rng();
        let u = rng.gen_bigint_range(&BigInt::from(0), &self.q);
        let v = rng.gen_bigint_range(&BigInt::from(0), &self.q);
        let w = rng.gen_bigint_range(&BigInt::from(0), &self.q);
        MediamText {
            value: &p.value * self.g.clone()
                + &u * self.h.clone()
                + &v * self.g.clone() * self.h.clone()
                + &w * self.h.clone() * self.g.clone(),
        }
    }

    fn m_to_p(&self, m: &MediamText) -> PlainText {
        PlainText {
            value: (2 * &m.value[0]) % &self.q,
        }
    }

    /// find G, H
    pub fn find_g_h(q: BigInt) -> (Octonion, Octonion) {
        // TODO: if using groebner basis
        let mut rng = rand::thread_rng();

        let two = BigInt::from(2);
        loop {
            let g0: BigInt = BigInt::from(1) * inverse(BigInt::from(2), q.clone());
            let g1 = rng.gen_bigint_range(&BigInt::from(0), &q);
            let g2 = rng.gen_bigint_range(&BigInt::from(0), &q);
            let g3 = rng.gen_bigint_range(&BigInt::from(0), &q);
            let g4 = rng.gen_bigint_range(&BigInt::from(0), &q);
            let g5 = rng.gen_bigint_range(&BigInt::from(0), &q);

            let h0: BigInt = BigInt::from(0);
            let h1 = rng.gen_bigint_range(&BigInt::from(0), &q);
            let h2 = rng.gen_bigint_range(&BigInt::from(0), &q);
            let h3 = rng.gen_bigint_range(&BigInt::from(0), &q);
            let h4 = rng.gen_bigint_range(&BigInt::from(0), &q);
            let h5 = rng.gen_bigint_range(&BigInt::from(0), &q);

            // find g6, g7, h6, h7
            let g6 = rng.gen_bigint_range(&BigInt::from(0), &q);

            let mut g7g7 = -(g0.modpow(&two, &q)
                + g1.modpow(&two, &q)
                + g2.modpow(&two, &q)
                + g3.modpow(&two, &q)
                + g4.modpow(&two, &q)
                + g5.modpow(&two, &q)
                + g6.modpow(&two, &q));
            g7g7 %= &q;
            if g7g7 < BigInt::default() {
                g7g7 += &q;
            }
            if !is_residue(&g7g7, &q) {
                // dbg!("omg", g7g7);
                continue;
            }

            // solve
            // h6h6 + h7h7 = b
            // g6h6 + g7h7 = d

            let mut b = -(h1.modpow(&two, &q)
                + h2.modpow(&two, &q)
                + h3.modpow(&two, &q)
                + h4.modpow(&two, &q)
                + h5.modpow(&two, &q));
            b %= &q;
            if b < BigInt::default() {
                b += &q;
            }

            let c = g6.clone();
            let c2 = c.modpow(&two, &q);

            let mut d = -(&g1 * &h1 + &g2 * &h2 + &g3 * &h3 + &g4 * &h4 + &g5 * &h5);
            d %= &q;
            if d < BigInt::default() {
                d += &q;
            }
            let d2 = d.modpow(&two, &q);

            let e2 = g7g7.clone();

            let mut pre_h7_key = &d2 * &e2 - (&e2 + &c2) * (&d2 - &b * &c2);
            pre_h7_key %= &q;
            if pre_h7_key < BigInt::default() {
                pre_h7_key += &q;
            }
            if !is_residue(&pre_h7_key, &q) {
                // dbg!("omg", pre_h7_key);
                continue;
            }

            let g7: BigInt = sqrt_with_mod(g7g7, q.clone()).unwrap();
            // dbg!(&g6, &g7);
            let e = g7.clone();

            let h7s = {
                let e2c2 = (&e2 + &c2) % &q;
                if e2c2 == BigInt::from(0) {
                    // e2 + c2 cant be divider
                    continue;
                }

                let sqrt = sqrt_with_mod(pre_h7_key, q.clone()).unwrap();
                let inv_e2c2 = inverse(e2c2, q.clone());
                let mut h7s = (&d * &e + &sqrt, &d * &e - &sqrt);
                h7s.0 %= &q;
                h7s.1 %= &q;
                if h7s.0 < BigInt::default() {
                    h7s.0 += &q;
                }
                if h7s.1 < BigInt::default() {
                    h7s.1 += &q;
                }
                h7s.0 *= &inv_e2c2;
                h7s.1 *= &inv_e2c2;
                h7s.0 %= &q;
                h7s.1 %= &q;
                if h7s.0 < BigInt::default() {
                    h7s.0 += &q;
                }
                if h7s.1 < BigInt::default() {
                    h7s.1 += &q;
                }
                h7s
            };

            let h6s = {
                let inv_c = inverse(c.clone(), q.clone());
                let mut h6s = ((&d - &h7s.0 * &e) * &inv_c, (&d - &h7s.1 * &e) * &inv_c);
                h6s.0 %= &q;
                h6s.1 %= &q;
                if h6s.0 < BigInt::default() {
                    h6s.0 += &q;
                }
                if h6s.1 < BigInt::default() {
                    h6s.1 += &q;
                }
                h6s
            };

            // dbg!(&h6s, &h7s);
            let g: Octonion = Octonion::new_with_bigint(g0, g1, g2, g3, g4, g5, g6, g7);
            let hs0: Octonion = Octonion::new_with_bigint(
                h0.clone(),
                h1.clone(),
                h2.clone(),
                h3.clone(),
                h4.clone(),
                h5.clone(),
                h6s.0,
                h7s.0,
            );
            // TODO: can i use something? zkp...
            let _: Octonion = Octonion::new_with_bigint(h0, h1, h2, h3, h4, h5, h6s.1, h7s.1);

            // break (g, hs1);
            break (g, hs0);
        }
    }
}
