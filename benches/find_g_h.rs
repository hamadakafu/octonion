#![feature(const_generics)]

use criterion::{criterion_group, criterion_main, Criterion};

use octonion::consts::{M2203_STR, M31_STR, M3217_STR, M521_STR, M9689_STR};
use octonion::crypto::Schema;

fn find_g_h<const MOD: &'static str>() {
    let _ = Schema::<MOD>::find_g_h();
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("find_g_h");
    group.bench_function("find_g_h M31", |b| b.iter(|| find_g_h::<M31_STR>()));
    group.bench_function("find_g_h M521", |b| b.iter(|| find_g_h::<M521_STR>()));
    group.bench_function("find_g_h M2203", |b| b.iter(|| find_g_h::<M2203_STR>()));
    group.bench_function("find_g_h M3217", |b| b.iter(|| find_g_h::<M3217_STR>()));
    group.bench_function("find_g_h M9689", |b| b.iter(|| find_g_h::<M9689_STR>()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
