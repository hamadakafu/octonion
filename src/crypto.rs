use crate::consts::M;
use core::panic;

use crate::types::Octonion;
use num_bigint::BigInt;
use num_bigint::RandBigInt;
use rand;

/// 八元数暗号系
pub struct Scheme {
    q: BigInt,
    g: Octonion,
    h: Octonion,
}

pub struct Plaintext {
    value: Octonion,
}

pub struct Mediamtext {
    value: Octonion,
    u: BigInt,
    v: BigInt,
    w: BigInt,
}

impl Scheme {
    pub fn new(q: BigInt, g: Octonion, h: Octonion) -> Self {
        // TODO: validate prime
        if q < BigInt::from(0) {
            panic!("modulus q({:?}) is less than 0", q);
        }
        Self { q, g, h }
    }

    pub fn new_plaintext(&self, p: Octonion) -> Plaintext {
        // TODO: pがFq上のOctonionになっているかをvalidate
        Plaintext { value: p }
    }

    /// plaintext -> mediamtext
    fn p_to_m(&self, p: Plaintext) -> Mediamtext {
        let mut rng = rand::thread_rng();
        let u = rng.gen_bigint_range(&BigInt::from(0), &M);
        let v = rng.gen_bigint_range(&BigInt::from(0), &M);
        let w = rng.gen_bigint_range(&BigInt::from(0), &M);
        Mediamtext {
            value: p.value * self.g.clone()
                + &u * self.h.clone()
                + &v * self.g.clone() * self.h.clone()
                + &w * self.h.clone() * self.g.clone(),
            u,
            v,
            w,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::quickcheck;
    // #[quickcheck]
    // fn test_mediamtext_assosiative(a: Octonion, b: Octonion) -> bool {
    //     Scheme::new(M, )
    // }
}
