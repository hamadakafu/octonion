use num_bigint::BigInt;
use num_bigint::RandBigInt;
use num_bigint::RandomBits;
use rand;
use rand::distributions::Distribution;

use crate::types::Octonion;
use crate::utils::inverse;
use crate::utils::is_residue;
use crate::utils::sqrt_with_mod;

/// 八元数暗号系
#[derive(Debug, Clone)]
pub struct Schema {
    q: BigInt,
    g: Octonion,
    h: Octonion,
}

#[derive(Debug, Clone)]
pub struct Plaintext {
    pub value: BigInt,
}

#[derive(Debug, Clone)]
pub struct Mediamtext {
    pub value: Octonion,
    pub u: BigInt,
    pub v: BigInt,
    pub w: BigInt,
}

impl Schema {
    pub fn new_with_q(q: BigInt, q_bit: u64) -> Self {
        let (g, h) = Self::find_g_h(q.clone(), q_bit);
        return Self { q, g, h };
    }

    pub fn new_with_q_g_h(q: BigInt, g: Octonion, h: Octonion) -> Self {
        // TODO: validate prime
        if q < BigInt::from(0) {
            panic!("modulus q({:?}) is less than 0", q);
        }
        Self { q, g, h }
    }

    pub fn new_plaintext(&self, p: BigInt) -> Plaintext {
        // TODO: pがFq上のOctonionになっているかをvalidate
        Plaintext { value: p }
    }

    /// plaintext -> mediamtext
    fn p_to_m(&self, p: Plaintext) -> Mediamtext {
        let mut rng = rand::thread_rng();
        let u = rng.gen_bigint_range(&BigInt::from(0), &self.q);
        let v = rng.gen_bigint_range(&BigInt::from(0), &self.q);
        let w = rng.gen_bigint_range(&BigInt::from(0), &self.q);
        Mediamtext {
            value: &p.value * self.g.clone()
                + &u * self.h.clone()
                + &v * self.g.clone() * self.h.clone()
                + &w * self.h.clone() * self.g.clone(),
            u,
            v,
            w,
        }
    }

    /// 条件を満たすG,Hを求める
    pub fn find_g_h(q: BigInt, q_bit: u64) -> (Octonion, Octonion) {
        // TODO: グレブナー基底でgとhを効率よく求められるかもしれない???
        let mut rng = rand::thread_rng();
        let M = q.clone();
        let M_BITS = q_bit;

        let two = BigInt::from(2);
        loop {
            let g0: BigInt = BigInt::from(1) * inverse(BigInt::from(2), M.clone());
            let mut g1: BigInt = RandomBits::new(M_BITS).sample(&mut rng);
            let mut g2: BigInt = RandomBits::new(M_BITS).sample(&mut rng);
            let mut g3: BigInt = RandomBits::new(M_BITS).sample(&mut rng);
            let mut g4: BigInt = RandomBits::new(M_BITS).sample(&mut rng);
            let mut g5: BigInt = RandomBits::new(M_BITS).sample(&mut rng);
            g1 %= &M;
            g2 %= &M;
            g3 %= &M;
            g4 %= &M;
            g5 %= &M;
            if g1 < BigInt::default() {
                g1 += &M;
            }
            if g2 < BigInt::default() {
                g2 += &M;
            }
            if g3 < BigInt::default() {
                g3 += &M;
            }
            if g4 < BigInt::default() {
                g4 += &M;
            }
            if g5 < BigInt::default() {
                g5 += &M;
            }

            let h0: BigInt = BigInt::from(0);
            let mut h1: BigInt = RandomBits::new(M_BITS).sample(&mut rng);
            let mut h2: BigInt = RandomBits::new(M_BITS).sample(&mut rng);
            let mut h3: BigInt = RandomBits::new(M_BITS).sample(&mut rng);
            let mut h4: BigInt = RandomBits::new(M_BITS).sample(&mut rng);
            let mut h5: BigInt = RandomBits::new(M_BITS).sample(&mut rng);

            h1 %= &M;
            h2 %= &M;
            h3 %= &M;
            h4 %= &M;
            h5 %= &M;

            if h1 < BigInt::default() {
                h1 += &M;
            }
            if h2 < BigInt::default() {
                h2 += &M;
            }
            if h3 < BigInt::default() {
                h3 += &M;
            }
            if h4 < BigInt::default() {
                h4 += &M;
            }
            if h5 < BigInt::default() {
                h5 += &M;
            }

            // g6, g7, h6, h7を決める

            // residueになるようにg6を決める
            let mut g6: BigInt = RandomBits::new(M_BITS).sample(&mut rng);
            g6 %= &M;
            if g6 < BigInt::default() {
                g6 += &M;
            }

            let mut g7g7 = -(g0.modpow(&two, &M)
                + g1.modpow(&two, &M)
                + g2.modpow(&two, &M)
                + g3.modpow(&two, &M)
                + g4.modpow(&two, &M)
                + g5.modpow(&two, &M)
                + g6.modpow(&two, &M));
            g7g7 %= &M;
            if g7g7 < BigInt::default() {
                g7g7 += &M;
            }

            // h6,h7の方程式を解く
            let mut b = -(h1.modpow(&two, &M)
                + h2.modpow(&two, &M)
                + h3.modpow(&two, &M)
                + h4.modpow(&two, &M)
                + h5.modpow(&two, &M));
            b %= &M;
            if b < BigInt::default() {
                b += &M;
            }

            let c = g6.clone();
            let c2 = c.modpow(&two, &M);

            let mut d = -(&g1 * &h1 + &g2 * &h2 + &g3 * &h3 + &g4 * &h4 + &g5 * &h5);
            d %= &M;
            if d < BigInt::default() {
                d += &M;
            }
            let d2 = d.modpow(&two, &M);

            // let e = g7.clone();
            let e2 = g7g7.clone();

            // h7が存在するのに満たす必要がある性質
            let mut pre_h7_key = &d2 * &e2 - (&e2 + &c2) * (&d2 - &b * &c2);
            pre_h7_key %= &M;
            if pre_h7_key < BigInt::default() {
                pre_h7_key += &M;
            }

            if !is_residue(&g7g7, &M) {
                // dbg!("omg", g7g7);
                continue;
            }
            if !is_residue(&pre_h7_key, &M) {
                // dbg!("omg", pre_h7_key);
                continue;
            }

            let g7: BigInt = sqrt_with_mod(g7g7, M.clone()).unwrap();
            // dbg!(&g6, &g7);
            let e = g7.clone();

            let h7s = {
                let sqrt = sqrt_with_mod(pre_h7_key, M.clone()).unwrap();
                let inv_e2c2 = inverse(&e2 + &c2, M.clone());
                let mut h7s = (&d * &e + &sqrt, &d * &e - &sqrt);
                h7s.0 %= &M;
                h7s.1 %= &M;
                if h7s.0 < BigInt::default() {
                    h7s.0 += &M;
                }
                if h7s.1 < BigInt::default() {
                    h7s.1 += &M;
                }
                h7s.0 *= &inv_e2c2;
                h7s.1 *= &inv_e2c2;
                h7s.0 %= &M;
                h7s.1 %= &M;
                if h7s.0 < BigInt::default() {
                    h7s.0 += &M;
                }
                if h7s.1 < BigInt::default() {
                    h7s.1 += &M;
                }
                h7s
            };

            let h6s = {
                let inv_c = inverse(c.clone(), M.clone());
                let mut h6s = ((&d - &h7s.0 * &e) * &inv_c, (&d - &h7s.1 * &e) * &inv_c);
                h6s.0 %= &M;
                h6s.1 %= &M;
                if h6s.0 < BigInt::default() {
                    h6s.0 += &M;
                }
                if h6s.1 < BigInt::default() {
                    h6s.1 += &M;
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
            let hs1: Octonion = Octonion::new_with_bigint(h0, h1, h2, h3, h4, h5, h6s.1, h7s.1);

            // break (g, hs1);
            break (g, hs0);
        }
    }
}

#[cfg(test)]
mod tests;
