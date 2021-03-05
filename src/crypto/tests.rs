use num_bigint::RandomBits;
use quickcheck::{Arbitrary, Gen};
use quickcheck_macros::quickcheck;
use rand::{prelude::Distribution, Rng};

use super::*;
use crate::consts::{M9689, M9689_BITS};

impl Arbitrary for Plaintext {
    fn arbitrary(_: &mut Gen) -> Self {
        let mut rng = rand::thread_rng();
        let mut p: BigInt = RandomBits::new(M9689_BITS).sample(&mut rng);
        p %= &*M9689;
        if p < BigInt::default() {
            p += &*M9689;
        }
        return Plaintext { value: p };
    }
}

#[quickcheck]
fn test_mediamtext_assosiative(a: Plaintext, b: Plaintext, c: Plaintext) -> bool {
    use crate::consts::{M9689, M9689_BITS};
    let schema = Schema::new_with_q(M9689.clone(), M9689_BITS);

    let am = schema.p_to_m(a);
    let bm = schema.p_to_m(b);
    let cm = schema.p_to_m(c);
    dbg!("o");
    am.value.clone() * (bm.value.clone() * cm.value.clone()) == (am.value * bm.value) * cm.value
}
