use std::{fmt::Display, fmt::Formatter, ops::Add, ops::Mul, str::FromStr};

use num_bigint::BigInt;

use crate::types::Octonion;

#[derive(Debug, Clone)]
pub struct CipherText<const MOD: &'static str> {
    /// coefficients f: O -> O
    /// e[i][x] ((e00, e01, ..., e07), ..., (e70, e71, ..., e77))
    pub e: Vec<Vec<BigInt>>,
}

impl<const MOD: &'static str> Display for CipherText<MOD> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "e:\n")?;
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

impl<'a, const MOD: &'static str> Add<&CipherText<MOD>> for &'a CipherText<MOD> {
    type Output = CipherText<MOD>;
    fn add(self, rhs: &CipherText<MOD>) -> CipherText<MOD> {
        let m = BigInt::from_str(MOD).unwrap();
        let enc_fn = |x: Octonion<MOD>| -> Octonion<MOD> {
            let mut ans = Octonion::zero();
            for ie in 0..8 {
                for ix in 0..8 {
                    ans[ie] += &self.e[ie][ix] * &x[ix];
                    ans[ie] += &rhs.e[ie][ix] * &x[ix];
                    ans[ie] %= &m;
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
        return CipherText { e };
    }
}

///E(E(X, M_2), M_1) = A_1 ... M_1 M_2 ... X
impl<'a, const MOD: &'static str> Mul<&CipherText<MOD>> for &'a CipherText<MOD> {
    type Output = CipherText<MOD>;
    fn mul(self, rhs: &CipherText<MOD>) -> CipherText<MOD> {
        let m = BigInt::from_str(MOD).unwrap();
        let enc_fn = |x: Octonion<MOD>| -> Octonion<MOD> {
            let mut ans_rhs: Octonion<MOD> = Octonion::zero();
            for ie in 0..8 {
                for ix in 0..8 {
                    ans_rhs[ie] += &rhs.e[ie][ix] * &x[ix];
                    ans_rhs[ie] %= &m;
                }
            }

            let mut ans_lhs = Octonion::zero();
            for ie in 0..8 {
                for ix in 0..8 {
                    ans_lhs[ie] += &self.e[ie][ix] * &ans_rhs[ix];
                    ans_lhs[ie] %= &m;
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
        return CipherText { e };
    }
}
