use num_bigint::{BigInt, Sign};
use num_bigint::{RandBigInt, ToBigInt};
use once_cell::sync::Lazy;
use rand::Rng;
use std::str::FromStr;

pub(crate) const m: Lazy<BigInt> =
    Lazy::new(|| BigInt::from_str("7140947329758317059570927").unwrap());

fn main() {
    let a = BigInt::from_str("7140947329758317059570927714094732975831705957092771409473297583170595709277140947329758317059570927714094732975831705957092771409473297583170595709277140947329758317059570927714094732975831705957092771409473297583170595709277140705419289473297583170595709277140947329758317059570927714094732975831705957092").unwrap();
    let b = BigInt::from_str("714094732975834710932472801479217405957092").unwrap();
    dbg!((a + b) % m.clone());

    let aa = BigInt::from_str("3").unwrap();
    let bb = BigInt::from_str("5").unwrap();
    let cc = BigInt::from_str("5").unwrap();
    dbg!((aa - bb) % cc == BigInt::from_str("-2").unwrap());
    let aa = BigInt::from_str("3").unwrap();
    let bb = BigInt::from_str("5").unwrap();
    let cc = BigInt::from_str("5").unwrap();
    dbg!((aa - bb) % cc > BigInt::new(Sign::Plus, vec![]));
    dbg!(BigInt::new(Sign::Minus, vec![]) == BigInt::default());
    dbg!(BigInt::from(0), BigInt::default());

    let mut rng = rand::thread_rng();
    let a = rng.gen_bigint(1000);

    let low = -10000.to_bigint().unwrap();
    let high = 10000.to_bigint().unwrap();
    let b = rng.gen_bigint_range(&low, &high);
    // Probably an even larger number.
    println!("a: {}, b: {}, a * b: {}", a.clone(), b.clone(), a * b);
    // dbg!(a * b);
}
