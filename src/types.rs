use std::{fmt::Formatter, str::FromStr};
use std::ops::{Index, IndexMut};
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use num_bigint;
use num_bigint::BigInt;

use crate::utils::inverse;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Octonion<const MOD: &'static str> {
    pub a0: BigInt,
    pub a1: BigInt,
    pub a2: BigInt,
    pub a3: BigInt,
    pub a4: BigInt,
    pub a5: BigInt,
    pub a6: BigInt,
    pub a7: BigInt,
}

impl<const MOD: &'static str> Octonion<MOD> {
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
        let m: BigInt = BigInt::from_str(MOD).unwrap();
        a0 %= &m;
        a1 %= &m;
        a2 %= &m;
        a3 %= &m;
        a4 %= &m;
        a5 %= &m;
        a6 %= &m;
        a7 %= &m;
        if a0 < BigInt::default() {
            a0 += &m;
        }
        if a1 < BigInt::default() {
            a1 += &m;
        }
        if a2 < BigInt::default() {
            a2 += &m;
        }
        if a3 < BigInt::default() {
            a3 += &m;
        }
        if a4 < BigInt::default() {
            a4 += &m;
        }
        if a5 < BigInt::default() {
            a5 += &m;
        }
        if a6 < BigInt::default() {
            a6 += &m;
        }
        if a7 < BigInt::default() {
            a7 += &m;
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
        let m = BigInt::from_str(MOD).unwrap();
        if !self.has_inv() {
            return None;
        }
        let norm2 = self.norm2();
        return Some(&inverse(norm2, m) * self.conjugate());
    }

    pub fn conjugate(&self) -> Self {
        let m = BigInt::from_str(MOD).unwrap();
        let mut c = Octonion::zero();
        c[0] = self.a0.clone();
        for i in 1..8 {
            c[i] = &m - &self[i];
            c[i] %= &m;
        }
        return c;
    }

    /// |self|^2
    pub fn norm2(&self) -> BigInt {
        let m = BigInt::from_str(MOD).unwrap();
        let two = BigInt::from(2);
        let mut norm2 = BigInt::from(0);
        for i in 0..8 {
            norm2 += self[i].modpow(&two, &m);
        }
        norm2 %= &m;
        return norm2;
    }
}

impl<const MOD: &'static str> Add for Octonion<MOD> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let m = BigInt::from_str(MOD).unwrap();
        return Self {
            a0: (self.a0 + rhs.a0) % &m,
            a1: (self.a1 + rhs.a1) % &m,
            a2: (self.a2 + rhs.a2) % &m,
            a3: (self.a3 + rhs.a3) % &m,
            a4: (self.a4 + rhs.a4) % &m,
            a5: (self.a5 + rhs.a5) % &m,
            a6: (self.a6 + rhs.a6) % &m,
            a7: (self.a7 + rhs.a7) % &m,
        };
    }
}

impl<const MOD: &'static str> AddAssign for Octonion<MOD> {
    fn add_assign(&mut self, rhs: Self) {
        let m = BigInt::from_str(MOD).unwrap();
        for i in 0..8 {
            self[i] += &rhs[i];
            self[i] %= &m;
        }
    }
}

impl<const MOD: &'static str> Sub for Octonion<MOD> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let m = BigInt::from_str(MOD).unwrap();
        let mut a0 = self.a0 - rhs.a0;
        let mut a1 = self.a1 - rhs.a1;
        let mut a2 = self.a2 - rhs.a2;
        let mut a3 = self.a3 - rhs.a3;
        let mut a4 = self.a4 - rhs.a4;
        let mut a5 = self.a5 - rhs.a5;
        let mut a6 = self.a6 - rhs.a6;
        let mut a7 = self.a7 - rhs.a7;
        a0 %= &m;
        a1 %= &m;
        a2 %= &m;
        a3 %= &m;
        a4 %= &m;
        a5 %= &m;
        a6 %= &m;
        a7 %= &m;
        if a0 < BigInt::default() {
            a0 += &m;
        }
        if a1 < BigInt::default() {
            a1 += &m;
        }
        if a2 < BigInt::default() {
            a2 += &m;
        }
        if a3 < BigInt::default() {
            a3 += &m;
        }
        if a4 < BigInt::default() {
            a4 += &m;
        }
        if a5 < BigInt::default() {
            a5 += &m;
        }
        if a6 < BigInt::default() {
            a6 += &m;
        }
        if a7 < BigInt::default() {
            a7 += &m;
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

impl<const MOD: &'static str> SubAssign for Octonion<MOD> {
    fn sub_assign(&mut self, rhs: Self) {
        let m = BigInt::from_str(MOD).unwrap();
        for i in 0..8 {
            self[i] -= &rhs[i];
            self[i] %= &m;
            if self[i] < BigInt::from(0) {
                self[i] += &m;
            }
        }
    }
}

impl<const MOD: &'static str> Mul for Octonion<MOD> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let m = BigInt::from_str(MOD).unwrap();
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

        a0 %= &m;
        a1 %= &m;
        a2 %= &m;
        a3 %= &m;
        a4 %= &m;
        a5 %= &m;
        a6 %= &m;
        a7 %= &m;
        if a0 < BigInt::from(0) {
            a0 += &m;
        }
        if a1 < BigInt::from(0) {
            a1 += &m;
        }
        if a2 < BigInt::from(0) {
            a2 += &m;
        }
        if a3 < BigInt::from(0) {
            a3 += &m;
        }
        if a4 < BigInt::from(0) {
            a4 += &m;
        }
        if a5 < BigInt::from(0) {
            a5 += &m;
        }
        if a6 < BigInt::from(0) {
            a6 += &m;
        }
        if a7 < BigInt::from(0) {
            a7 += &m;
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

impl<const MOD: &'static str> MulAssign for Octonion<MOD> {
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

impl<const MOD: &'static str> Mul<Octonion<MOD>> for &BigInt {
    type Output = Octonion<MOD>;
    fn mul(self, rhs: Octonion<MOD>) -> Self::Output {
        let m = BigInt::from_str(MOD).unwrap();
        let mut ans = Octonion::zero();
        for i in 0..8 {
            ans[i] = (&rhs[i] * self) % &m;
            if ans[i] < BigInt::from(0) {
                ans[i] += &m;
            }
        }
        return ans;
    }
}

impl<const MOD: &'static str> Div for Octonion<MOD> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let m = BigInt::from_str(MOD).unwrap();
        let divider = (self.a0.pow(2)
            + self.a1.pow(2)
            + self.a2.pow(2)
            + self.a3.pow(2)
            + self.a4.pow(2)
            + self.a5.pow(2)
            + self.a6.pow(2)
            + self.a7.pow(2))
            % &m;
        if divider == BigInt::from(0) {
            panic!("{:?} is zero, can't div.", self);
        }

        let rhs_inv = Self {
            a0: (rhs.a0 * inverse(divider.clone(), m.clone())) % &m,
            a1: ((&m - rhs.a1) * inverse(divider.clone(), m.clone())) % &m,
            a2: ((&m - rhs.a2) * inverse(divider.clone(), m.clone())) % &m,
            a3: ((&m - rhs.a3) * inverse(divider.clone(), m.clone())) % &m,
            a4: ((&m - rhs.a4) * inverse(divider.clone(), m.clone())) % &m,
            a5: ((&m - rhs.a5) * inverse(divider.clone(), m.clone())) % &m,
            a6: ((&m - rhs.a6) * inverse(divider.clone(), m.clone())) % &m,
            a7: ((&m - rhs.a7) * inverse(divider.clone(), m.clone())) % &m,
        };
        self * rhs_inv
    }
}

impl<const MOD: &'static str> DivAssign for Octonion<MOD> {
    fn div_assign(&mut self, rhs: Self) {
        let m = BigInt::from_str(MOD).unwrap();
        let divider = (self.a0.pow(2)
            + self.a1.pow(2)
            + self.a2.pow(2)
            + self.a3.pow(2)
            + self.a4.pow(2)
            + self.a5.pow(2)
            + self.a6.pow(2)
            + self.a7.pow(2))
            % &m;
        if divider == BigInt::from(0) {
            panic!("{:?} is zero, can't div.", self);
        }

        let rhs_inv = Self {
            a0: (rhs.a0 * inverse(divider.clone(), m.clone())) % &m,
            a1: (rhs.a1 * inverse(divider.clone(), m.clone())) % &m,
            a2: (rhs.a2 * inverse(divider.clone(), m.clone())) % &m,
            a3: (rhs.a3 * inverse(divider.clone(), m.clone())) % &m,
            a4: (rhs.a4 * inverse(divider.clone(), m.clone())) % &m,
            a5: (rhs.a5 * inverse(divider.clone(), m.clone())) % &m,
            a6: (rhs.a6 * inverse(divider.clone(), m.clone())) % &m,
            a7: (rhs.a7 * inverse(divider.clone(), m.clone())) % &m,
        };
        *self *= rhs_inv;
    }
}

impl<const MOD: &'static str> Index<usize> for Octonion<MOD> {
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

impl<const MOD: &'static str> IndexMut<usize> for Octonion<MOD> {
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

impl<const MOD: &'static str> Display for Octonion<MOD> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "O: ")?;
        for i in 0..8 {
            write!(f, "{}", self[i])?;
        }
        Ok(())
    }
}
