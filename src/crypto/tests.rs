use num_bigint::RandBigInt;
use quickcheck::{Arbitrary, Gen};
use quickcheck_macros::quickcheck;

use super::*;
use crate::consts::M521_STR;

const M: &'static str = M521_STR;

impl<const MOD: &'static str> Arbitrary for PlainText<MOD> {
    fn arbitrary(_: &mut Gen) -> Self {
        let m = BigInt::from_str(MOD).unwrap();
        let mut rng = rand::thread_rng();
        let p: BigInt = rng.gen_bigint_range(&BigInt::from(0), &m);
        return PlainText { value: p };
    }
}

#[quickcheck]
fn test_mediamtext_assosiative(a: PlainText<M>, b: PlainText<M>, c: PlainText<M>) -> bool {
    let schema = Schema::new();
    let am = schema.p_to_m(a);
    let bm = schema.p_to_m(b);
    let cm = schema.p_to_m(c);
    am.value.clone() * (bm.value.clone() * cm.value.clone()) == (am.value * bm.value) * cm.value
}

#[quickcheck]
fn test_encrypt_decrypt(pt: PlainText<M>) -> bool {
    let schema = Schema::new();
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
fn test_encrypt_decrypt_add(lhs_pt: PlainText<M>, rhs_pt: PlainText<M>) -> bool {
    let m = BigInt::from_str(M).unwrap();

    let schema = Schema::new();
    let (sk, pk) = schema.gen_sk_pk();
    let lhs_ct = schema.encrypt(lhs_pt.clone(), &pk);
    let rhs_ct = schema.encrypt(rhs_pt.clone(), &pk);
    let add_ct = &lhs_ct + &rhs_ct;
    let add_pt_hat = schema.decrypt(add_ct, &sk);
    let ans = (&lhs_pt.value + &rhs_pt.value) % &m;

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
fn test_encrypt_decrypt_mul(lhs_pt: PlainText<M>, rhs_pt: PlainText<M>) -> bool {
    let m = BigInt::from_str(M).unwrap();

    let schema = Schema::new();
    let (sk, pk) = schema.gen_sk_pk();
    let lhs_ct = schema.encrypt(lhs_pt.clone(), &pk);
    let rhs_ct = schema.encrypt(rhs_pt.clone(), &pk);
    assert_eq!(schema.decrypt(lhs_ct.clone(), &sk).value, lhs_pt.value);
    assert_eq!(schema.decrypt(rhs_ct.clone(), &sk).value, rhs_pt.value);
    let mul_ct = &lhs_ct * &rhs_ct;
    let mul_pt_hat = schema.decrypt(mul_ct, &sk);
    let ans = (&lhs_pt.value * &rhs_pt.value) % &m;

    if ans != mul_pt_hat.value {
        println!("sk: {}", sk);
        println!("pk: {}", pk);
        println!("pl: {}, pr: {}", lhs_pt.value, rhs_pt.value);
        println!("mul_pt_hat.value: {}", &mul_pt_hat.value);
    } else {
        println!(
            "ok, {} * {}  == {}",
            lhs_pt.value, rhs_pt.value, mul_pt_hat.value
        );
    }
    ans == mul_pt_hat.value
}
