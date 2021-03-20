#![feature(const_generics)]

use criterion::{criterion_group, criterion_main, Criterion};
use num_bigint::BigInt;
use octonion::crypto::SecretKey;

use octonion::{
    consts::{M31_STR, M3217_STR, M521_STR, M9689_STR},
    crypto::{cipher_text::CipherText, PlainText, Schema},
};

fn mul<const MOD: &'static str>(ctl: &CipherText<MOD>, ctr: &CipherText<MOD>) {
    ctl * ctr;
}

fn setup<const MOD: &'static str>(schema: &Schema<MOD>) -> (CipherText<MOD>, CipherText<MOD>) {
    let (sk, pk) = schema.gen_sk_pk();
    let ptl = PlainText {
        value: BigInt::from(100000000),
    };
    let ptr = PlainText {
        value: BigInt::from(200000000),
    };
    let ctl = schema.encrypt(ptl, &pk);
    let ctr = schema.encrypt(ptr, &pk);
    return (ctl, ctr);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("mul");

    let schema_m31 = Schema::<M31_STR>::new();
    let schema_m521 = Schema::<M521_STR>::new();
    let schema_m3217 = Schema::<M3217_STR>::new();
    let schema_m9689 = Schema::<M9689_STR>::new();
    let (ctl_m31, ctr_m31) = setup(&schema_m31);
    let (ctl_m521, ctr_m521) = setup(&schema_m521);
    let (ctl_m3217, ctr_m3217) = setup(&schema_m3217);
    let (ctl_m9689, ctr_m9689) = setup(&schema_m9689);

    group.bench_function("mul M31", |b| b.iter(|| mul(&ctl_m31, &ctr_m31)));
    group.bench_function("mul M521", |b| b.iter(|| mul(&ctl_m521, &ctr_m521)));
    group.bench_function("mul M3217", |b| b.iter(|| mul(&ctl_m3217, &ctr_m3217)));
    group.bench_function("mul M9689", |b| b.iter(|| mul(&ctl_m9689, &ctr_m9689)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
