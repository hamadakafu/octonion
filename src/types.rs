use std::fmt::Formatter;
use std::ops::{Index, IndexMut};
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use num_bigint;
use num_bigint::BigInt;

use crate::consts::M;
use crate::utils::inverse;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Octonion {
    pub a0: BigInt,
    pub a1: BigInt,
    pub a2: BigInt,
    pub a3: BigInt,
    pub a4: BigInt,
    pub a5: BigInt,
    pub a6: BigInt,
    pub a7: BigInt,
}

impl Octonion {
    pub fn new_with_bigint(
        mut a0: BigInt,
        mut a1: BigInt,
        mut a2: BigInt,
        mut a3: BigInt,
        mut a4: BigInt,
        mut a5: BigInt,
        mut a6: BigInt,
        mut a7: BigInt,
    ) -> Self {
        a0 %= &*M;
        a1 %= &*M;
        a2 %= &*M;
        a3 %= &*M;
        a4 %= &*M;
        a5 %= &*M;
        a6 %= &*M;
        a7 %= &*M;
        if a0 < BigInt::default() {
            a0 += &*M;
        }
        if a1 < BigInt::default() {
            a1 += &*M;
        }
        if a2 < BigInt::default() {
            a2 += &*M;
        }
        if a3 < BigInt::default() {
            a3 += &*M;
        }
        if a4 < BigInt::default() {
            a4 += &*M;
        }
        if a5 < BigInt::default() {
            a5 += &*M;
        }
        if a6 < BigInt::default() {
            a6 += &*M;
        }
        if a7 < BigInt::default() {
            a7 += &*M;
        }
        Self {
            a0,
            a1,
            a2,
            a3,
            a4,
            a5,
            a6,
            a7,
        }
    }

    pub fn zero() -> Self {
        Octonion::new_with_bigint(
            BigInt::from(0),
            BigInt::from(0),
            BigInt::from(0),
            BigInt::from(0),
            BigInt::from(0),
            BigInt::from(0),
            BigInt::from(0),
            BigInt::from(0),
        )
    }

    /// identity element
    pub fn one() -> Self {
        Octonion::new_with_bigint(
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

    /// all one
    pub fn ones() -> Self {
        Octonion::new_with_bigint(
            BigInt::from(1),
            BigInt::from(1),
            BigInt::from(1),
            BigInt::from(1),
            BigInt::from(1),
            BigInt::from(1),
            BigInt::from(1),
            BigInt::from(1),
        )
    }

    pub fn is_zero(&self) -> bool {
        let zero = BigInt::from(0);
        self.a0 == zero
            && self.a1 == zero
            && self.a2 == zero
            && self.a3 == zero
            && self.a4 == zero
            && self.a5 == zero
            && self.a6 == zero
            && self.a7 == zero
    }

    pub fn has_inv(&self) -> bool {
        let norm2 = self.norm2();
        norm2 != BigInt::from(0)
    }

    /// if exists, self.conjugate / |a|^2
    pub fn inverse(&self) -> Option<Self> {
        if !self.has_inv() {
            return None;
        }
        let norm2 = self.norm2();
        return Some(&inverse(norm2, M.clone()) * self.conjugate());
    }

    pub fn conjugate(&self) -> Self {
        let mut a1 = &*M - &self.a1;
        let mut a2 = &*M - &self.a2;
        let mut a3 = &*M - &self.a3;
        let mut a4 = &*M - &self.a4;
        let mut a5 = &*M - &self.a5;
        let mut a6 = &*M - &self.a6;
        let mut a7 = &*M - &self.a7;
        a1 %= &*M;
        a2 %= &*M;
        a3 %= &*M;
        a4 %= &*M;
        a5 %= &*M;
        a6 %= &*M;
        a7 %= &*M;
        Self {
            a0: self.a0.clone(),
            a1,
            a2,
            a3,
            a4,
            a5,
            a6,
            a7,
        }
    }

    /// |self|^2
    pub fn norm2(&self) -> BigInt {
        let two = BigInt::from(2);
        (self.a0.modpow(&two, &*M)
            + self.a1.modpow(&two, &*M)
            + self.a2.modpow(&two, &*M)
            + self.a3.modpow(&two, &*M)
            + self.a4.modpow(&two, &*M)
            + self.a5.modpow(&two, &*M)
            + self.a6.modpow(&two, &*M)
            + self.a7.modpow(&two, &*M))
            % &*M
    }
}

impl Add for Octonion {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        return Self {
            a0: (self.a0 + rhs.a0) % &*M,
            a1: (self.a1 + rhs.a1) % &*M,
            a2: (self.a2 + rhs.a2) % &*M,
            a3: (self.a3 + rhs.a3) % &*M,
            a4: (self.a4 + rhs.a4) % &*M,
            a5: (self.a5 + rhs.a5) % &*M,
            a6: (self.a6 + rhs.a6) % &*M,
            a7: (self.a7 + rhs.a7) % &*M,
        };
    }
}

impl AddAssign for Octonion {
    fn add_assign(&mut self, rhs: Self) {
        self.a0 += rhs.a0;
        self.a1 += rhs.a1;
        self.a2 += rhs.a2;
        self.a3 += rhs.a3;
        self.a4 += rhs.a4;
        self.a5 += rhs.a5;
        self.a6 += rhs.a6;
        self.a7 += rhs.a7;

        self.a0 %= &*M;
        self.a1 %= &*M;
        self.a2 %= &*M;
        self.a3 %= &*M;
        self.a4 %= &*M;
        self.a5 %= &*M;
        self.a6 %= &*M;
        self.a7 %= &*M;
    }
}

impl Sub for Octonion {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut a0 = self.a0 - rhs.a0;
        let mut a1 = self.a1 - rhs.a1;
        let mut a2 = self.a2 - rhs.a2;
        let mut a3 = self.a3 - rhs.a3;
        let mut a4 = self.a4 - rhs.a4;
        let mut a5 = self.a5 - rhs.a5;
        let mut a6 = self.a6 - rhs.a6;
        let mut a7 = self.a7 - rhs.a7;
        a0 %= &*M;
        a1 %= &*M;
        a2 %= &*M;
        a3 %= &*M;
        a4 %= &*M;
        a5 %= &*M;
        a6 %= &*M;
        a7 %= &*M;
        if a0 < BigInt::default() {
            a0 += &*M;
        }
        if a1 < BigInt::default() {
            a1 += &*M;
        }
        if a2 < BigInt::default() {
            a2 += &*M;
        }
        if a3 < BigInt::default() {
            a3 += &*M;
        }
        if a4 < BigInt::default() {
            a4 += &*M;
        }
        if a5 < BigInt::default() {
            a5 += &*M;
        }
        if a6 < BigInt::default() {
            a6 += &*M;
        }
        if a7 < BigInt::default() {
            a7 += &*M;
        }
        Self {
            a0,
            a1,
            a2,
            a3,
            a4,
            a5,
            a6,
            a7,
        }
    }
}

impl SubAssign for Octonion {
    fn sub_assign(&mut self, rhs: Self) {
        self.a0 -= rhs.a0;
        self.a1 -= rhs.a1;
        self.a2 -= rhs.a2;
        self.a3 -= rhs.a3;
        self.a4 -= rhs.a4;
        self.a5 -= rhs.a5;
        self.a6 -= rhs.a6;
        self.a7 -= rhs.a7;
        self.a0 %= &*M;
        self.a1 %= &*M;
        self.a2 %= &*M;
        self.a3 %= &*M;
        self.a4 %= &*M;
        self.a5 %= &*M;
        self.a6 %= &*M;
        self.a7 %= &*M;
        if self.a0 < BigInt::default() {
            self.a0 += &*M;
        }
        if self.a1 < BigInt::default() {
            self.a1 += &*M;
        }
        if self.a2 < BigInt::default() {
            self.a2 += &*M;
        }
        if self.a3 < BigInt::default() {
            self.a3 += &*M;
        }
        if self.a4 < BigInt::default() {
            self.a4 += &*M;
        }
        if self.a5 < BigInt::default() {
            self.a5 += &*M;
        }
        if self.a6 < BigInt::default() {
            self.a6 += &*M;
        }
        if self.a7 < BigInt::default() {
            self.a7 += &*M;
        }
    }
}

impl Mul for Octonion {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut a0 = self.a0.clone() * rhs.a0.clone()
            - self.a1.clone() * rhs.a1.clone()
            - self.a2.clone() * rhs.a2.clone()
            - self.a3.clone() * rhs.a3.clone()
            - self.a4.clone() * rhs.a4.clone()
            - self.a5.clone() * rhs.a5.clone()
            - self.a6.clone() * rhs.a6.clone()
            - self.a7.clone() * rhs.a7.clone();
        let mut a1 = self.a0.clone() * rhs.a1.clone()
            + self.a1.clone() * rhs.a0.clone()
            + self.a2.clone() * rhs.a3.clone()
            - self.a3.clone() * rhs.a2.clone()
            + self.a4.clone() * rhs.a5.clone()
            - self.a5.clone() * rhs.a4.clone()
            + self.a7.clone() * rhs.a6.clone()
            - self.a6.clone() * rhs.a7.clone();
        let mut a2 = self.a0.clone() * rhs.a2.clone() + self.a2.clone() * rhs.a0.clone()
            - self.a1.clone() * rhs.a3.clone()
            + self.a3.clone() * rhs.a1.clone()
            + self.a4.clone() * rhs.a6.clone()
            - self.a6.clone() * rhs.a4.clone()
            + self.a5.clone() * rhs.a7.clone()
            - self.a7.clone() * rhs.a5.clone();
        let mut a3 = self.a0.clone() * rhs.a3.clone()
            + self.a3.clone() * rhs.a0.clone()
            + self.a1.clone() * rhs.a2.clone()
            - self.a2.clone() * rhs.a1.clone()
            + self.a4.clone() * rhs.a7.clone()
            - self.a7.clone() * rhs.a4.clone()
            - self.a5.clone() * rhs.a6.clone()
            + self.a6.clone() * rhs.a5.clone();
        let mut a4 = self.a0.clone() * rhs.a4.clone() + self.a4.clone() * rhs.a0.clone()
            - self.a1.clone() * rhs.a5.clone()
            + self.a5.clone() * rhs.a1.clone()
            - self.a2.clone() * rhs.a6.clone()
            + self.a6.clone() * rhs.a2.clone()
            - self.a3.clone() * rhs.a7.clone()
            + self.a7.clone() * rhs.a3.clone();
        let mut a5 = self.a0.clone() * rhs.a5.clone()
            + self.a5.clone() * rhs.a0.clone()
            + self.a1.clone() * rhs.a4.clone()
            - self.a4.clone() * rhs.a1.clone()
            - self.a2.clone() * rhs.a7.clone()
            + self.a7.clone() * rhs.a2.clone()
            + self.a3.clone() * rhs.a6.clone()
            - self.a6.clone() * rhs.a3.clone();
        let mut a6 = self.a0.clone() * rhs.a6.clone()
            + self.a6.clone() * rhs.a0.clone()
            + self.a1.clone() * rhs.a7.clone()
            - self.a7.clone() * rhs.a1.clone()
            + self.a2.clone() * rhs.a4.clone()
            - self.a4.clone() * rhs.a2.clone()
            - self.a3.clone() * rhs.a5.clone()
            + self.a5.clone() * rhs.a3.clone();
        let mut a7 = self.a0.clone() * rhs.a7.clone() + self.a7.clone() * rhs.a0.clone()
            - self.a1.clone() * rhs.a6.clone()
            + self.a6.clone() * rhs.a1.clone()
            + self.a2.clone() * rhs.a5.clone()
            - self.a5.clone() * rhs.a2.clone()
            + self.a3.clone() * rhs.a4.clone()
            - self.a4.clone() * rhs.a3.clone();

        a0 %= &*M;
        a1 %= &*M;
        a2 %= &*M;
        a3 %= &*M;
        a4 %= &*M;
        a5 %= &*M;
        a6 %= &*M;
        a7 %= &*M;
        if a0 < BigInt::from(0) {
            a0 += &*M;
        }
        if a1 < BigInt::from(0) {
            a1 += &*M;
        }
        if a2 < BigInt::from(0) {
            a2 += &*M;
        }
        if a3 < BigInt::from(0) {
            a3 += &*M;
        }
        if a4 < BigInt::from(0) {
            a4 += &*M;
        }
        if a5 < BigInt::from(0) {
            a5 += &*M;
        }
        if a6 < BigInt::from(0) {
            a6 += &*M;
        }
        if a7 < BigInt::from(0) {
            a7 += &*M;
        }

        Self {
            a0,
            a1,
            a2,
            a3,
            a4,
            a5,
            a6,
            a7,
        }
    }
}

impl MulAssign for Octonion {
    fn mul_assign(&mut self, rhs: Self) {
        let tmp = self.clone() * rhs;
        self.a0 = tmp.a0;
        self.a1 = tmp.a1;
        self.a2 = tmp.a2;
        self.a3 = tmp.a3;
        self.a4 = tmp.a4;
        self.a5 = tmp.a5;
        self.a6 = tmp.a6;
        self.a7 = tmp.a7;
    }
}

impl Mul<Octonion> for &BigInt {
    type Output = Octonion;
    fn mul(self, rhs: Octonion) -> Self::Output {
        let mut a0 = (rhs.a0 * self) % &*M;
        let mut a1 = (rhs.a1 * self) % &*M;
        let mut a2 = (rhs.a2 * self) % &*M;
        let mut a3 = (rhs.a3 * self) % &*M;
        let mut a4 = (rhs.a4 * self) % &*M;
        let mut a5 = (rhs.a5 * self) % &*M;
        let mut a6 = (rhs.a6 * self) % &*M;
        let mut a7 = (rhs.a7 * self) % &*M;
        if a0 < BigInt::from(0) {
            a0 += &*M;
        }
        if a1 < BigInt::from(0) {
            a1 += &*M;
        }
        if a2 < BigInt::from(0) {
            a2 += &*M;
        }
        if a3 < BigInt::from(0) {
            a3 += &*M;
        }
        if a4 < BigInt::from(0) {
            a4 += &*M;
        }
        if a5 < BigInt::from(0) {
            a5 += &*M;
        }
        if a6 < BigInt::from(0) {
            a6 += &*M;
        }
        if a7 < BigInt::from(0) {
            a7 += &*M;
        }
        Self::Output {
            a0,
            a1,
            a2,
            a3,
            a4,
            a5,
            a6,
            a7,
        }
    }
}

impl Div for Octonion {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let divider = (self.a0.pow(2)
            + self.a1.pow(2)
            + self.a2.pow(2)
            + self.a3.pow(2)
            + self.a4.pow(2)
            + self.a5.pow(2)
            + self.a6.pow(2)
            + self.a7.pow(2))
            % &*M;
        if divider == BigInt::from(0) {
            panic!("{:?} is zero, can't div.", self);
        }

        let rhs_inv = Self {
            a0: (rhs.a0 * inverse(divider.clone(), M.clone())) % &*M,
            a1: ((&*M - rhs.a1) * inverse(divider.clone(), M.clone())) % &*M,
            a2: ((&*M - rhs.a2) * inverse(divider.clone(), M.clone())) % &*M,
            a3: ((&*M - rhs.a3) * inverse(divider.clone(), M.clone())) % &*M,
            a4: ((&*M - rhs.a4) * inverse(divider.clone(), M.clone())) % &*M,
            a5: ((&*M - rhs.a5) * inverse(divider.clone(), M.clone())) % &*M,
            a6: ((&*M - rhs.a6) * inverse(divider.clone(), M.clone())) % &*M,
            a7: ((&*M - rhs.a7) * inverse(divider.clone(), M.clone())) % &*M,
        };
        self * rhs_inv
    }
}

impl DivAssign for Octonion {
    fn div_assign(&mut self, rhs: Self) {
        let divider = (self.a0.pow(2)
            + self.a1.pow(2)
            + self.a2.pow(2)
            + self.a3.pow(2)
            + self.a4.pow(2)
            + self.a5.pow(2)
            + self.a6.pow(2)
            + self.a7.pow(2))
            % &*M;
        if divider == BigInt::from(0) {
            panic!("{:?} is zero, can't div.", self);
        }

        let rhs_inv = Self {
            a0: (rhs.a0 * inverse(divider.clone(), M.clone())) % &*M,
            a1: (rhs.a1 * inverse(divider.clone(), M.clone())) % &*M,
            a2: (rhs.a2 * inverse(divider.clone(), M.clone())) % &*M,
            a3: (rhs.a3 * inverse(divider.clone(), M.clone())) % &*M,
            a4: (rhs.a4 * inverse(divider.clone(), M.clone())) % &*M,
            a5: (rhs.a5 * inverse(divider.clone(), M.clone())) % &*M,
            a6: (rhs.a6 * inverse(divider.clone(), M.clone())) % &*M,
            a7: (rhs.a7 * inverse(divider.clone(), M.clone())) % &*M,
        };
        *self *= rhs_inv;
    }
}

impl Index<usize> for Octonion {
    type Output = BigInt;
    fn index(&self, idx: usize) -> &<Self as Index<usize>>::Output {
        match idx {
            0 => &self.a0,
            1 => &self.a1,
            2 => &self.a2,
            3 => &self.a3,
            4 => &self.a4,
            5 => &self.a5,
            6 => &self.a6,
            7 => &self.a7,
            _ => panic!("index {} out of bounds.", idx),
        }
    }
}

impl IndexMut<usize> for Octonion {
    fn index_mut(&mut self, idx: usize) -> &mut <Self as Index<usize>>::Output {
        match idx {
            0 => &mut self.a0,
            1 => &mut self.a1,
            2 => &mut self.a2,
            3 => &mut self.a3,
            4 => &mut self.a4,
            5 => &mut self.a5,
            6 => &mut self.a6,
            7 => &mut self.a7,
            _ => panic!("index {} out of bounds.", idx),
        }
    }
}

impl Display for Octonion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "O: ")?;
        for i in 0..8 {
            write!(f, "{}", self[i])?;
        }
        Ok(())
    }
}
