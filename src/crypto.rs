use std::fmt::Formatter;
use std::{fmt::Display, str::FromStr};

use num_bigint::BigInt;
use num_bigint::RandBigInt;
use rand;

use crate::crypto::cipher_text::CipherText;
use crate::types::Octonion;
use crate::utils::gen_rand_octonion_which_has_inv;
use crate::utils::inverse;
use crate::utils::is_residue;
use crate::utils::sqrt_with_mod;

pub mod cipher_text;
#[cfg(test)]
mod tests;

/// octonion schema
#[derive(Debug, Clone)]
pub struct Schema<const MOD: &'static str> {
    g: Octonion<MOD>,
    h: Octonion<MOD>,
}

#[derive(Debug, Clone)]
pub struct PlainText<const MOD: &'static str> {
    pub value: BigInt,
}

#[derive(Debug, Clone)]
pub struct MediamText<const MOD: &'static str> {
    pub value: Octonion<MOD>,
}

impl<const MOD: &'static str> Display for MediamText<MOD> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.value)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct SecretKey<const MOD: &'static str> {
    /// Aのlength
    pub h: usize,
    /// A_i
    pub a: Vec<Octonion<MOD>>,
}

impl<const MOD: &'static str> Display for SecretKey<MOD> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "h: {}, a:", self.h)?;
        for a in self.a.iter() {
            write!(f, " {}", a)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct PublicKey<const MOD: &'static str> {
    /// coefficients f: (O \times O) -> O
    /// ijk -> i x y ((e000 * x0 * y0 + ... + e077 * x7 * y7), ...)
    pub e: Vec<Vec<Vec<BigInt>>>,
}

impl<const MOD: &'static str> Display for PublicKey<MOD> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "e:")?;
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

impl<const MOD: &'static str> PublicKey<MOD> {
    /// generate public key from secret key
    pub fn new_from_sk(sk: &SecretKey<MOD>) -> PublicKey<MOD> {
        let enc_fn = |x: Octonion<MOD>, y: Octonion<MOD>| {
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
        return PublicKey { e };
    }
}

impl<const MOD: &'static str> Schema<MOD> {
    pub fn new() -> Self {
        let (g, h) = Self::find_g_h();
        return Self::new_with_g_h(g, h);
    }

    pub fn new_with_g_h(g: Octonion<MOD>, h: Octonion<MOD>) -> Self {
        Self { g, h }
    }

    pub fn new_plaintext(&self, p: BigInt) -> PlainText<MOD> {
        // TODO: validate Octonion over Fq
        let m = BigInt::from_str(MOD).unwrap();
        PlainText { value: p % m }
    }

    /// generate secret key and public key
    pub fn gen_sk_pk(&self) -> (SecretKey<MOD>, PublicKey<MOD>) {
        // TODO: how to determin SecretKey.a.len()
        let h = 56;
        let a = {
            let mut a = Vec::with_capacity(h);
            for _ in 0..h {
                a.push(gen_rand_octonion_which_has_inv());
            }
            a
        };
        let sk = SecretKey { h, a };
        let pk = PublicKey::new_from_sk(&sk);
        return (sk, pk);
    }

    pub fn encrypt(&self, pt: PlainText<MOD>, pk: &PublicKey<MOD>) -> CipherText<MOD> {
        let m = BigInt::from_str(MOD).unwrap();

        let mt = self.p_to_m(pt);
        let mut e = vec![vec![BigInt::from(0); 8]; 8];
        for ie in 0..8 {
            for ix in 0..8 {
                for iy in 0..8 {
                    e[ie][ix] += &pk.e[ie][ix][iy] * &mt.value[iy];
                    e[ie][ix] %= &m;
                }
            }
        }
        CipherText { e }
    }

    pub fn decrypt(&self, ct: CipherText<MOD>, sk: &SecretKey<MOD>) -> PlainText<MOD> {
        let m = BigInt::from_str(MOD).unwrap();

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
            mt[ie] %= &m;
        }

        // A_r^-1 ( ... (A_1^-1 pt) )
        for a in sk.a.iter() {
            mt = a.inverse().unwrap() * mt;
        }

        self.m_to_p(&MediamText { value: mt })
    }

    /// plaintext -> mediamtext
    pub fn p_to_m(&self, p: PlainText<MOD>) -> MediamText<MOD> {
        let m = BigInt::from_str(MOD).unwrap();

        let mut rng = rand::thread_rng();
        let u = rng.gen_bigint_range(&BigInt::from(0), &m);
        let v = rng.gen_bigint_range(&BigInt::from(0), &m);
        let w = rng.gen_bigint_range(&BigInt::from(0), &m);
        MediamText {
            value: &p.value * self.g.clone()
                + &u * self.h.clone()
                + &v * self.g.clone() * self.h.clone()
                + &w * self.h.clone() * self.g.clone(),
        }
    }

    pub fn m_to_p(&self, mt: &MediamText<MOD>) -> PlainText<MOD> {
        let m = BigInt::from_str(MOD).unwrap();

        PlainText {
            value: (2 * &mt.value[0]) % &m,
        }
    }

    /// find G, H
    pub fn find_g_h() -> (Octonion<MOD>, Octonion<MOD>) {
        let m = BigInt::from_str(MOD).unwrap();
        // TODO: if using groebner basis
        let mut rng = rand::thread_rng();

        let two = BigInt::from(2);
        loop {
            let g0: BigInt = BigInt::from(1) * inverse(BigInt::from(2), m.clone());
            let g1 = rng.gen_bigint_range(&BigInt::from(0), &m);
            let g2 = rng.gen_bigint_range(&BigInt::from(0), &m);
            let g3 = rng.gen_bigint_range(&BigInt::from(0), &m);
            let g4 = rng.gen_bigint_range(&BigInt::from(0), &m);
            let g5 = rng.gen_bigint_range(&BigInt::from(0), &m);

            let h0: BigInt = BigInt::from(0);
            let h1 = rng.gen_bigint_range(&BigInt::from(0), &m);
            let h2 = rng.gen_bigint_range(&BigInt::from(0), &m);
            let h3 = rng.gen_bigint_range(&BigInt::from(0), &m);
            let h4 = rng.gen_bigint_range(&BigInt::from(0), &m);
            let h5 = rng.gen_bigint_range(&BigInt::from(0), &m);

            // find g6, g7, h6, h7
            let g6 = rng.gen_bigint_range(&BigInt::from(0), &m);

            let mut g7g7 = -(g0.modpow(&two, &m)
                + g1.modpow(&two, &m)
                + g2.modpow(&two, &m)
                + g3.modpow(&two, &m)
                + g4.modpow(&two, &m)
                + g5.modpow(&two, &m)
                + g6.modpow(&two, &m));
            g7g7 %= &m;
            if g7g7 < BigInt::default() {
                g7g7 += &m;
            }
            if !is_residue(&g7g7, &m) {
                // dbg!("omg", g7g7);
                continue;
            }

            // solve
            // h6h6 + h7h7 = b
            // g6h6 + g7h7 = d

            let mut b = -(h1.modpow(&two, &m)
                + h2.modpow(&two, &m)
                + h3.modpow(&two, &m)
                + h4.modpow(&two, &m)
                + h5.modpow(&two, &m));
            b %= &m;
            if b < BigInt::default() {
                b += &m;
            }

            let c = g6.clone();
            let c2 = c.modpow(&two, &m);

            let mut d = -(&g1 * &h1 + &g2 * &h2 + &g3 * &h3 + &g4 * &h4 + &g5 * &h5);
            d %= &m;
            if d < BigInt::default() {
                d += &m;
            }
            let d2 = d.modpow(&two, &m);

            let e2 = g7g7.clone();

            let mut pre_h7_key = &d2 * &e2 - (&e2 + &c2) * (&d2 - &b * &c2);
            pre_h7_key %= &m;
            if pre_h7_key < BigInt::default() {
                pre_h7_key += &m;
            }
            if !is_residue(&pre_h7_key, &m) {
                // dbg!("omg", pre_h7_key);
                continue;
            }

            let g7: BigInt = sqrt_with_mod(g7g7, m.clone()).unwrap();
            // dbg!(&g6, &g7);
            let e = g7.clone();

            let h7s = {
                let e2c2 = (&e2 + &c2) % &m;
                if e2c2 == BigInt::from(0) {
                    // e2 + c2 cant be divider
                    continue;
                }

                let sqrt = sqrt_with_mod(pre_h7_key, m.clone()).unwrap();
                let inv_e2c2 = inverse(e2c2, m.clone());
                let mut h7s = (&d * &e + &sqrt, &d * &e - &sqrt);
                h7s.0 %= &m;
                h7s.1 %= &m;
                if h7s.0 < BigInt::default() {
                    h7s.0 += &m;
                }
                if h7s.1 < BigInt::default() {
                    h7s.1 += &m;
                }
                h7s.0 *= &inv_e2c2;
                h7s.1 *= &inv_e2c2;
                h7s.0 %= &m;
                h7s.1 %= &m;
                if h7s.0 < BigInt::default() {
                    h7s.0 += &m;
                }
                if h7s.1 < BigInt::default() {
                    h7s.1 += &m;
                }
                h7s
            };

            let h6s = {
                let inv_c = inverse(c.clone(), m.clone());
                let mut h6s = ((&d - &h7s.0 * &e) * &inv_c, (&d - &h7s.1 * &e) * &inv_c);
                h6s.0 %= &m;
                h6s.1 %= &m;
                if h6s.0 < BigInt::default() {
                    h6s.0 += &m;
                }
                if h6s.1 < BigInt::default() {
                    h6s.1 += &m;
                }
                h6s
            };

            // dbg!(&h6s, &h7s);
            let g: Octonion<MOD> = Octonion::new_with_bigint(g0, g1, g2, g3, g4, g5, g6, g7);
            let hs0: Octonion<MOD> = Octonion::new_with_bigint(
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
            let _: Octonion<MOD> = Octonion::new_with_bigint(h0, h1, h2, h3, h4, h5, h6s.1, h7s.1);

            // break (g, hs1);
            break (g, hs0);
        }
    }
}
