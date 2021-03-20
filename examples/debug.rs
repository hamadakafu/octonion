use num_bigint::BigInt;
use octonion::crypto::Schema;
use octonion::crypto::SecretKey;
use octonion::crypto::{PlainText, PublicKey};
use octonion::types::Octonion;

fn main() {
    let schema = Schema::new_with_q(BigInt::from(5));
    // schema.debug();
    // let q = BigInt::from(5);
    // let aa = Octonion::new_with_bigint(
    //     BigInt::from(0),
    //     BigInt::from(3),
    //     BigInt::from(2),
    //     BigInt::from(4),
    //     BigInt::from(0),
    //     BigInt::from(2),
    //     BigInt::from(3),
    //     BigInt::from(4),
    // );
    // let sk = SecretKey {
    //     q,
    //     h: 1,
    //     a: vec![aa.clone()],
    // };
    // let pk = new_from_sk(&sk);
    // println!("sk: {}, pk: {}", sk, pk);
    // let ct = schema.encrypt(
    //     PlainText {
    //         value: BigInt::from(3),
    //     },
    //     &pk,
    // );
    // let mt = Octonion::new_with_bigint(
    //     BigInt::from(4),
    //     BigInt::from(4),
    //     BigInt::from(3),
    //     BigInt::from(2),
    //     BigInt::from(0),
    //     BigInt::from(2),
    //     BigInt::from(0),
    //     BigInt::from(4),
    // );
    // let mt2 = Octonion::new_with_bigint(
    //     BigInt::from(4),
    //     BigInt::from(4),
    //     BigInt::from(3),
    //     BigInt::from(2),
    //     BigInt::from(0),
    //     BigInt::from(2),
    //     BigInt::from(0),
    //     BigInt::from(4),
    // );
    // let ans = aa.clone().inverse().unwrap()
    //     * (aa.clone()
    //         * (mt2.clone()
    //             * (mt * (aa.clone() * (aa.clone().inverse().unwrap() * Octonion::ones())))));
    // println!("ans: {}", ans);
    // println!("ct: {}", ct);
    // let pt_hat = schema.decrypt(ct, &sk);
    // println!("pt: {}", pt_hat.value);
}
fn new_from_sk(sk: &SecretKey) -> PublicKey {
    let enc_fn = |x: Octonion, y: Octonion| {
        let mut ans = x;

        // A_h^-1 ( ... ( A_1^-1 X ) )
        for a in &sk.a {
            ans = a.inverse().unwrap() * ans;
        }

        ans = y * ans;

        // A_1 ( ... ( A_h ans ) )
        for a in sk.a.iter().rev() {
            ans = a.clone() * ans;
        }
        return ans;
    };

    let mut e = vec![vec![vec![BigInt::from(0); 8]; 8]; 8];
    for ix in 0..8 {
        for iy in 0..8 {
            let mut x = Octonion::zero();
            let mut y = Octonion::zero();
            x[ix] = BigInt::from(1);
            y[iy] = BigInt::from(1);
            let result = enc_fn(x, y);
            for ie in 0..8 {
                e[ie][ix][iy] = result[ie].clone();
            }
        }
    }
    return PublicKey {
        q: sk.q.clone(),
        e,
    };
}
