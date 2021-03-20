use num_bigint::RandBigInt;
use once_cell::sync::Lazy;
use quickcheck::{Arbitrary, Gen};
use quickcheck_macros::quickcheck;

use super::*;
use crate::consts::{M, M_BITS};

impl Arbitrary for PlainText {
    fn arbitrary(_: &mut Gen) -> Self {
        let mut rng = rand::thread_rng();
        let p: BigInt = rng.gen_bigint_range(&BigInt::from(0), &*M);
        return PlainText { value: p };
    }
}

#[quickcheck]
fn test_mediamtext_assosiative(a: PlainText, b: PlainText, c: PlainText) -> bool {
    let schema = Schema::new_with_q(M.clone(), M_BITS);
    let am = schema.p_to_m(a);
    let bm = schema.p_to_m(b);
    let cm = schema.p_to_m(c);
    am.value.clone() * (bm.value.clone() * cm.value.clone()) == (am.value * bm.value) * cm.value
}

#[quickcheck]
fn test_encrypt_decrypt(pt: PlainText) -> bool {
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

#[quickcheck]
fn test_encrypt_decrypt_add(lhs_pt: PlainText, rhs_pt: PlainText) -> bool {
    let schema = Schema::new_with_q(M.clone(), M_BITS);
    let (sk, pk) = schema.gen_sk_pk();
    let lhs_ct = schema.encrypt(lhs_pt.clone(), &pk);
    let rhs_ct = schema.encrypt(rhs_pt.clone(), &pk);
    let add_ct = &lhs_ct + &rhs_ct;
    let add_pt_hat = schema.decrypt(add_ct, &sk);
    let ans = (&lhs_pt.value + &rhs_pt.value) % M.clone();

    if ans != add_pt_hat.value {
        println!("sk: {}", sk);
        println!("pk: {}", pk);
        println!("ans.value: {}", &ans);
        println!("add_pt_hat.value: {}", &add_pt_hat.value);
    } else {
        println!(
            "ok, {} + {} == {}",
            lhs_pt.value, rhs_pt.value, add_pt_hat.value
        );
    }
    ans == add_pt_hat.value
}

#[quickcheck]
fn test_encrypt_decrypt_mul(lhs_pt: PlainText, rhs_pt: PlainText) -> bool {
    let schema = Schema::new_with_q(M.clone(), M_BITS);
    let (sk, pk) = schema.gen_sk_pk();
    let lhs_ct = schema.encrypt(lhs_pt.clone(), &pk);
    let rhs_ct = schema.encrypt(rhs_pt.clone(), &pk);
    assert_eq!(schema.decrypt(lhs_ct.clone(), &sk).value, lhs_pt.value);
    assert_eq!(schema.decrypt(rhs_ct.clone(), &sk).value, rhs_pt.value);
    let mul_ct = &lhs_ct * &rhs_ct;
    let mul_pt_hat = schema.decrypt(mul_ct, &sk);
    let ans = (&lhs_pt.value * &rhs_pt.value) % M.clone();

    if ans != mul_pt_hat.value {
        println!("sk: {}", sk);
        println!("pk: {}", pk);
        println!("pl: {}, pr: {}", lhs_pt.value, rhs_pt.value);
        println!("mul_pt_hat.value: {}", &mul_pt_hat.value);
    } else {
        println!("ok, {} * {}  == {}", lhs_pt.value, rhs_pt.value, mul_pt_hat.value);
    }
    ans == mul_pt_hat.value
}
