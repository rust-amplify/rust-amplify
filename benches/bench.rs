#[macro_use]
extern crate criterion;

use criterion::{black_box, Criterion};

use amplify_num::u1024;
use amplify_num::posit::Posit32;
use softposit::P32;

fn criterion_u1024(c: &mut Criterion) {
    const X: u1024 = u1024::from_inner([
        0xe4, 0xf7, 0xab, 0x7f, 0xdd, 0xaa, 0xbb, 0xcc, 0x77, 0x66, 0xf5, 0x06, 0x39, 0xa2, 0xb8,
        0xcc,
    ]);
    const Y: u1024 = u1024::from_inner([
        0xd2, 0x77, 0x66, 0xf5, 0x06, 0x39, 0xee, 0xa6, 0xf7, 0xab, 0x7f, 0xc2, 0x55, 0x75, 0x90,
        0x21,
    ]);

    c.bench_function("u1024_add", |c| {
        c.iter(|| black_box(X).overflowing_add(black_box(Y)))
    });
    c.bench_function("u1024_sub", |c| {
        c.iter(|| black_box(X).overflowing_sub(black_box(Y)))
    });
    c.bench_function("u1024_mul", |c| {
        c.iter(|| black_box(X).overflowing_mul(black_box(Y)))
    });
    c.bench_function("u1024_div", |c| {
        c.iter(|| black_box(X).overflowing_div(black_box(Y)))
    });
}

fn criterion_posit32(c: &mut Criterion) {
    let x = Posit32::from(12.5);
    let y = Posit32::from(117.334);

    c.bench_function("posit32_add", move |c| {
        c.iter(|| black_box(x) + black_box(y))
    });
    c.bench_function("posit32_sub", move |c| {
        c.iter(|| black_box(x) - black_box(y))
    });
    c.bench_function("posit32_mul", move |c| {
        c.iter(|| black_box(x) * black_box(y))
    });
    c.bench_function("posit32_div", move |c| {
        c.iter(|| black_box(x) / black_box(y))
    });
}

fn criterion_softposit32(c: &mut Criterion) {
    let x = P32::from(12.5);
    let y = P32::from(117.334);

    c.bench_function("softposit32_add", move |c| {
        c.iter(|| black_box(x) + black_box(y))
    });
    c.bench_function("softposit32_sub", move |c| {
        c.iter(|| black_box(x) - black_box(y))
    });
    c.bench_function("softposit32_mul", move |c| {
        c.iter(|| black_box(x) * black_box(y))
    });
    c.bench_function("softposit32_div", move |c| {
        c.iter(|| black_box(x) / black_box(y))
    });
}

criterion_group!(
    benches,
    criterion_u1024,
    criterion_posit32,
    criterion_softposit32
);
criterion_main!(benches);
