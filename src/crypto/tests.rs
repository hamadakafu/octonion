use num_bigint::RandBigInt;
use quickcheck::{Arbitrary, Gen};
use quickcheck_macros::quickcheck;

use super::*;
use crate::consts::{M, M_BITS};

impl Arbitrary for Plaintext {
    fn arbitrary(_: &mut Gen) -> Self {
        let mut rng = rand::thread_rng();
        let p: BigInt = rng.gen_bigint_range(&BigInt::from(0), &*M);
        return Plaintext { value: p };
    }
}

#[quickcheck]
fn test_mediamtext_assosiative(a: Plaintext, b: Plaintext, c: Plaintext) -> bool {
    let schema = Schema::new_with_q(M.clone(), M_BITS);

    let am = schema.p_to_m(a);
    let bm = schema.p_to_m(b);
    let cm = schema.p_to_m(c);
    am.value.clone() * (bm.value.clone() * cm.value.clone()) == (am.value * bm.value) * cm.value
}

#[quickcheck]
fn test_encrypt_decrypt(pt: Plaintext) -> bool {
    let schema = Schema::new_with_q(M.clone(), M_BITS);
    let (sk, pk) = schema.gen_sk_pk();
    let ct = schema.encrypt(pt.clone(), &pk);
    let pt_hat = schema.decrypt(ct, &sk);
    if pt.value != pt_hat.value {
        println!("sk: {}", sk);
        println!("pk: {}", pk);
        println!("pt.value: {}", &pt.value);
        println!("pt_hat.value: {}", &pt_hat.value);
    } else {
        println!("ok, {} == {}", pt.value, pt_hat.value);
    }
    pt.value == pt_hat.value
}
