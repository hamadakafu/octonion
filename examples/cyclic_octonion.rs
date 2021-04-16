use std::collections::HashMap;

use num_bigint::BigInt;
use octonion::types::Octonion;

fn main() {
    let a = Octonion::<"5"> {
        a0: BigInt::from(4),
        a1: BigInt::from(1),
        a2: BigInt::from(1),
        a3: BigInt::from(2),
        a4: BigInt::from(2),
        a5: BigInt::from(3),
        a6: BigInt::from(4),
        a7: BigInt::from(2),
    };
    println!("{}", a.has_inv());

    let mut options = HashMap::new();
    options.insert(a.clone(), 1);

    let mut count = 2;
    loop {
        let mut target = Octonion::one();
        for _ in 0..count {
            target = target * a.clone();
        }
        if let Some(_) = options.insert(target.clone(), count) {
            break;
        }
        count += 1;
        println!("increment {} {}", count, target);
    }
    for (k, v) in options {
        println!("{}: {}", k, v);
    }
    println!("count: {}", count);

    // cyclic
    // println!("{:#?}", (b.clone() * a.clone()) / a.clone());
}
