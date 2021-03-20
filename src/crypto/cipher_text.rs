use std::{fmt::Display, fmt::Formatter, ops::Add, ops::Mul};

use num_bigint::BigInt;

use crate::types::Octonion;

#[derive(Debug, Clone)]
pub struct CipherText {
    pub q: BigInt,
    pub q_bits: u64,
    /// coefficients f: O -> O
    /// e[i][x] ((e00, e01, ..., e07), ..., (e70, e71, ..., e77))
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

impl<'a> Add<&CipherText> for &'a CipherText {
    type Output = CipherText;
    fn add(self, rhs: &CipherText) -> CipherText {
        let enc_fn = |x: Octonion| -> Octonion {
            let mut ans = Octonion::zero();
            for ie in 0..8 {
                for ix in 0..8 {
                    ans[ie] += &self.e[ie][ix] * &x[ix];
                    ans[ie] += &rhs.e[ie][ix] * &x[ix];
                    ans[ie] %= &self.q;
                }
            }
            return ans;
        };

        let mut e = vec![vec![BigInt::from(0); 8]; 8];
        for ix in 0..8 {
            let mut x = Octonion::zero();
            x[ix] = BigInt::from(1);
            let ans = enc_fn(x);
            for ie in 0..8 {
                e[ie][ix] = ans[ie].clone();
            }
        }
        return CipherText {
            q: self.q.clone(),
            q_bits: self.q_bits,
            e,
        };
    }
}

///E(E(X, M_2), M_1) = A_1 ... M_1 M_2 ... X
impl<'a> Mul<&CipherText> for &'a CipherText {
    type Output = CipherText;
    fn mul(self, rhs: &CipherText) -> CipherText {
        let enc_fn = |x: Octonion| -> Octonion {
            let mut ans_rhs = Octonion::zero();
            for ie in 0..8 {
                for ix in 0..8 {
                    ans_rhs[ie] += &rhs.e[ie][ix] * &x[ix];
                    ans_rhs[ie] %= &self.q;
                }
            }

            let mut ans_lhs = Octonion::zero();
            for ie in 0..8 {
                for ix in 0..8 {
                    ans_lhs[ie] += &self.e[ie][ix] * &ans_rhs[ix];
                    ans_lhs[ie] %= &self.q;
                }
            }
            return ans_lhs;
        };

        let mut e = vec![vec![BigInt::from(0); 8]; 8];
        for ix in 0..8 {
            let mut x = Octonion::zero();
            x[ix] = BigInt::from(1);
            let ans = enc_fn(x);
            for ie in 0..8 {
                e[ie][ix] = ans[ie].clone();
            }
        }
        return CipherText {
            q: self.q.clone(),
            q_bits: self.q_bits,
            e,
        };
    }
}
