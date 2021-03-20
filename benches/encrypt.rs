#![feature(const_generics)]

use criterion::{criterion_group, criterion_main, Criterion};
use num_bigint::BigInt;

use octonion::{
    consts::{M31_STR, M3217_STR, M521_STR, M9689_STR},
    crypto::{PlainText, PublicKey, Schema},
};

fn encrypt<const MOD: &'static str>(schema: &Schema<MOD>, pt: PlainText<MOD>, pk: &PublicKey<MOD>) {
    let _ = schema.encrypt(pt, &pk);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("encrypt");

    let schema_m31 = Schema::<M31_STR>::new();
    let schema_m521 = Schema::<M521_STR>::new();
    let schema_m3217 = Schema::<M3217_STR>::new();
    let schema_m9689 = Schema::<M9689_STR>::new();
    let (_, pk_m31) = schema_m31.gen_sk_pk();
    let (_, pk_m521) = schema_m521.gen_sk_pk();
    let (_, pk_m3217) = schema_m3217.gen_sk_pk();
    let (_, pk_m9689) = schema_m9689.gen_sk_pk();

    group.bench_function("encrypt M31", |b| {
        b.iter(|| {
            encrypt(
                &schema_m31,
                PlainText {
                    value: BigInt::from(100000),
                },
                &pk_m31,
            )
        })
    });
    group.bench_function("encrypt M521", |b| {
        b.iter(|| {
            encrypt(
                &schema_m521,
                PlainText {
                    value: BigInt::from(100000),
                },
                &pk_m521,
            )
        })
    });
    group.bench_function("encrypt M3217", |b| {
        b.iter(|| {
            encrypt(
                &schema_m3217,
                PlainText {
                    value: BigInt::from(100000),
                },
                &pk_m3217,
            )
        })
    });
    group.bench_function("encrypt M9689", |b| {
        b.iter(|| {
            encrypt(
                &schema_m9689,
                PlainText {
                    value: BigInt::from(100000),
                },
                &pk_m9689,
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
