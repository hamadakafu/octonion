#![feature(const_generics)]
use num_bigint::BigInt;
use octonion::crypto::Schema;
use octonion::crypto::SecretKey;
use octonion::crypto::{MediamText, PlainText, PublicKey};
use octonion::types::Octonion;

fn main() {
    let schema = Schema::<"5">::new();
    schema.gen_sk_pk();
    let three = schema.new_plaintext(BigInt::from(3));
    let three_m = schema.p_to_m(three);
    println!("{}", three_m.value.clone() * three_m.value.clone());
    let threethree = three_m.value.clone() * three_m.value.clone();
    let threethree_m = MediamText { value: threethree };
    println!("{}", schema.m_to_p(&threethree_m).value);
}
