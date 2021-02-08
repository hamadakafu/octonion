use num_bigint::BigInt;
use octonion::types::Octonion;

fn main() {
    let a = Octonion {
        a0: BigInt::from(0),
        a1: BigInt::from(0),
        a2: BigInt::from(0),
        a3: BigInt::from(2),
        a4: BigInt::from(2),
        a5: BigInt::from(1),
        a6: BigInt::from(4),
        a7: BigInt::from(0),
    };
    let b = Octonion {
        a0: BigInt::from(4),
        a1: BigInt::from(2),
        a2: BigInt::from(4),
        a3: BigInt::from(3),
        a4: BigInt::from(1),
        a5: BigInt::from(2),
        a6: BigInt::from(2),
        a7: BigInt::from(0),
    };
    println!("{:#?}", b.clone() * a.clone());
    println!("{:#?}", (b.clone() * a.clone()) / a.clone());
}
