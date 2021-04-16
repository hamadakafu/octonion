#![feature(const_generics)]

use std::str::FromStr;

use criterion::{criterion_group, criterion_main, Criterion};
use num_bigint::BigInt;
use num_bigint::RandBigInt;

use octonion::{
    consts::{M2203_STR, M31_STR, M3217_STR, M521_STR, M9689_STR},
    crypto::{cipher_text::CipherText, PlainText, Schema},
};

fn add<const MOD: &'static str>(ctl: &CipherText<MOD>, ctr: &CipherText<MOD>) {
    ctl + ctr;
}

fn setup<const MOD: &'static str>(schema: &Schema<MOD>) -> (CipherText<MOD>, CipherText<MOD>) {
    let (_, pk) = schema.gen_sk_pk();

    let mut rng = rand::thread_rng();
    let ptl = PlainText {
        value: rng.gen_bigint_range(&BigInt::from(0), &BigInt::from_str(MOD).unwrap()),
    };
    let ptr = PlainText {
        value: rng.gen_bigint_range(&BigInt::from(0), &BigInt::from_str(MOD).unwrap()),
    };
    let ctl = schema.encrypt(ptl, &pk);
    let ctr = schema.encrypt(ptr, &pk);
    return (ctl, ctr);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("add");

    let schema_m31 = Schema::<M31_STR>::new();
    let schema_m521 = Schema::<M521_STR>::new();
    let schema_m2203 = Schema::<M2203_STR>::new();
    let schema_m3217 = Schema::<M3217_STR>::new();
    let schema_m9689 = Schema::<M9689_STR>::new();
    let (ctl_m31, ctr_m31) = setup(&schema_m31);
    let (ctl_m521, ctr_m521) = setup(&schema_m521);
    let (ctl_m2203, ctr_m2203) = setup(&&schema_m2203);
    let (ctl_m3217, ctr_m3217) = setup(&schema_m3217);
    let (ctl_m9689, ctr_m9689) = setup(&schema_m9689);

    group.bench_function("add M31", |b| b.iter(|| add(&ctl_m31, &ctr_m31)));
    group.bench_function("add M521", |b| b.iter(|| add(&ctl_m521, &ctr_m521)));
    group.bench_function("add M2203", |b| b.iter(|| add(&ctl_m2203, &ctr_m2203)));
    group.bench_function("add M3217", |b| b.iter(|| add(&ctl_m3217, &ctr_m3217)));
    group.bench_function("add M9689", |b| b.iter(|| add(&ctl_m9689, &ctr_m9689)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
