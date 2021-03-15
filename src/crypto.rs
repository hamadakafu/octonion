use std::fmt::Display;
use std::fmt::Formatter;

use num_bigint::BigInt;
use num_bigint::RandBigInt;
use rand;

use crate::types::Octonion;
use crate::utils::gen_rand_octonion_which_has_inv;
use crate::utils::inverse;
use crate::utils::is_residue;
use crate::utils::sqrt_with_mod;

/// 八元数暗号系
#[derive(Debug, Clone)]
pub struct Schema {
    q: BigInt,
    q_bits: u64,
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
    // TODO: uvwはprivateにすべきかもしれない
    // TODO: uvwは保存する必要がない
    pub u: BigInt,
    pub v: BigInt,
    pub w: BigInt,
}

impl Display for Mediamtext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.value)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CipherText {
    pub q: BigInt,
    pub q_bits: u64,
    /// f: O -> O の係数
    /// e[i][x] の順で指定する
    pub e: Vec<Vec<BigInt>>,
}

impl Display for CipherText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "q: {}, e:\n", self.q)?;
        for ie in 0..8 {
            write!(f, "ie{}", ie)?;
            for ix in 0..8 {
                write!(f, " {}", self.e[ie][ix])?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct SecretKey {
    pub q: BigInt,
    pub q_bits: u64,
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
    pub q_bits: u64,
    /// ijk -> i x y の順で指定する
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
            q_bits: sk.q_bits,
            e,
        };
    }
}

impl Schema {
    pub fn new_with_q(q: BigInt, q_bits: u64) -> Self {
        let (g, h) = Self::find_g_h(q.clone(), q_bits);
        // TODO: validate prime
        if q < BigInt::from(0) {
            panic!("modulus q({:?}) is less than 0", q);
        }
        return Self::new_with_q_g_h(q, q_bits, g, h);
    }

    pub fn new_with_q_g_h(q: BigInt, q_bits: u64, g: Octonion, h: Octonion) -> Self {
        Self { q, q_bits, g, h }
    }

    pub fn new_plaintext(&self, p: BigInt) -> Plaintext {
        // TODO: pがFq上のOctonionになっているかをvalidate
        Plaintext { value: p }
    }

    /// generate secret key and public key
    pub fn gen_sk_pk(&self) -> (SecretKey, PublicKey) {
        // TODO: 秘密鍵に使うAの個数はとりあえず定数
        let h = 1;
        let a = {
            let mut a = Vec::with_capacity(h);
            for _ in 0..h {
                a.push(gen_rand_octonion_which_has_inv(&self.q, self.q_bits));
            }
            a
        };
        let sk = SecretKey {
            h,
            a,
            q: self.q.clone(),
            q_bits: self.q_bits,
        };
        let pk = PublicKey::new_from_sk(&sk);
        return (sk, pk);
    }

    pub fn encrypt(&self, pt: Plaintext, pk: &PublicKey) -> CipherText {
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
            q_bits: self.q_bits,
            e,
        }
    }

    pub fn decrypt(&self, ct: CipherText, sk: &SecretKey) -> Plaintext {
        // TODO: feature(fn_traits)ができるようになったら、impl Fn for SecretKey
        let mut pt = Octonion::zero();
        let mut x = Octonion::one();
        // A_1 ( ... (A_h 1) )
        for a in sk.a.iter().rev() {
            x = a.clone() * x;
        }

        for ie in 0..8 {
            for ix in 0..8 {
                pt[ie] += &ct.e[ie][ix] * &x[ix];
            }
            pt[ie] %= &self.q;
        }

        // A_r^-1 ( ... (A_1^-1 pt) )
        for a in sk.a.iter() {
            pt = a.inverse().unwrap() * pt;
        }

        Plaintext {
            value: (2 * &pt[0]) % &self.q,
        }
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

            // g6, g7, h6, h7を決める

            // residueになるようにg6を決める
            let g6 = rng.gen_bigint_range(&BigInt::from(0), &q);

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
                let e2c2 = (&e2 + &c2) % &M;
                if e2c2 == BigInt::from(0) {
                    // e2 + c2で割れないのでcontinue
                    continue;
                }

                let sqrt = sqrt_with_mod(pre_h7_key, M.clone()).unwrap();
                let inv_e2c2 = inverse(e2c2, M.clone());
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
            // TODO: あまりのHをなにかにつかえないか?
            let _: Octonion = Octonion::new_with_bigint(h0, h1, h2, h3, h4, h5, h6s.1, h7s.1);

            // break (g, hs1);
            break (g, hs0);
        }
    }
}

#[cfg(test)]
mod tests;
