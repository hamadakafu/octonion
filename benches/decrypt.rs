#![feature(const_generics)]

use criterion::{criterion_group, criterion_main, Criterion};
use num_bigint::BigInt;
use octonion::crypto::SecretKey;

use octonion::{
    consts::{M31_STR, M3217_STR, M521_STR, M9689_STR},
    crypto::{cipher_text::CipherText, PlainText, Schema},
};

fn decrypt<const MOD: &'static str>(
    schema: &Schema<MOD>,
    ct: CipherText<MOD>,
    sk: &SecretKey<MOD>,
) {
    let _ = schema.decrypt(ct, &sk);
}

fn setup<const MOD: &'static str>(schema: &Schema<MOD>) -> (CipherText<MOD>, SecretKey<MOD>) {
    let (sk, pk) = schema.gen_sk_pk();
    let pt = PlainText {
        value: BigInt::from(100000000),
    };
    let ct = schema.encrypt(pt, &pk);
    return (ct, sk);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("decrypt");

    let schema_m31 = Schema::<M31_STR>::new();
    let schema_m521 = Schema::<M521_STR>::new();
    let schema_m3217 = Schema::<M3217_STR>::new();
    let schema_m9689 = Schema::<M9689_STR>::new();
    let (ct_m31, sk_m31) = setup(&schema_m31);
    let (ct_m521, sk_m521) = setup(&schema_m521);
    let (ct_m3217, sk_m3217) = setup(&schema_m3217);
    let (ct_m9689, sk_m9689) = setup(&schema_m9689);

    group.bench_function("decrypt M31", |b| {
        b.iter(|| decrypt(&schema_m31, ct_m31.clone(), &sk_m31))
    });
    group.bench_function("decrypt M521", |b| {
        b.iter(|| decrypt(&schema_m521, ct_m521.clone(), &sk_m521))
    });
    group.bench_function("decrypt M3217", |b| {
        b.iter(|| decrypt(&schema_m3217, ct_m3217.clone(), &sk_m3217))
    });
    group.bench_function("decrypt M9689", |b| {
        b.iter(|| decrypt(&schema_m9689, ct_m9689.clone(), &sk_m9689))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
