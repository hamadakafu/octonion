#![feature(const_generics)]

use criterion::{criterion_group, criterion_main, Criterion};

use octonion::consts::{M31_STR, M3217_STR, M521_STR, M9689_STR};
use octonion::crypto::Schema;

fn find_g_h_m31() {
    let _ = Schema::<M31_STR>::find_g_h();
}
fn find_g_h_m521() {
    let _ = Schema::<M521_STR>::find_g_h();
}
fn find_g_h_m3217() {
    let _ = Schema::<M3217_STR>::find_g_h();
}
fn find_g_h_m9689() {
    let _ = Schema::<M9689_STR>::find_g_h();
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("find_g_h");
    group.bench_function("find_g_h M31", |b| b.iter(|| find_g_h_m31()));
    group.bench_function("find_g_h M521", |b| b.iter(|| find_g_h_m521()));
    group.bench_function("find_g_h M3217", |b| b.iter(|| find_g_h_m3217()));
    group.bench_function("find_g_h M9689", |b| b.iter(|| find_g_h_m9689()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
