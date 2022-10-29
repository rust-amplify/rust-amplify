#[macro_use]
extern crate criterion;

use criterion::{black_box, Criterion};

use amplify_num::{i1024, u1024};
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
    c.bench_function("u1024_cmp_eq", |c| {
        c.iter(|| black_box(X).cmp(black_box(&X)))
    });
}

fn criterion_i1024(c: &mut Criterion) {
    const X: i1024 = i1024::from_inner([
        0xe4, 0xf7, 0xab, 0x7f, 0xdd, 0xaa, 0xbb, 0xcc, 0x77, 0x66, 0xf5, 0x06, 0x39, 0xa2, 0xb8,
        0xcc,
    ]);
    const Y: i1024 = i1024::from_inner([
        0xd2, 0x77, 0x66, 0xf5, 0x06, 0x39, 0xee, 0xa6, 0xf7, 0xab, 0x7f, 0xc2, 0x55, 0x75, 0x90,
        0x21,
    ]);
    const Z: i1024 = i1024::from_inner([
        0xd2,
        0x77,
        0x66,
        0xf5,
        0x06,
        0x39,
        0xee,
        0xa6,
        0xf7,
        0xab,
        0x7f,
        0xc2,
        0x55,
        0x75,
        0x90,
        0xffff_ffff_ffff_ff21,
    ]);
    assert!(X.is_positive());
    assert!(Y.is_positive());
    assert!(Z.is_negative());

    c.bench_function("i1024_add", |c| {
        c.iter(|| black_box(X).overflowing_add(black_box(Y)))
    });
    c.bench_function("i1024_is_positive", |c| {
        c.iter(|| black_box(X).is_positive())
    });
    c.bench_function("i1024_is_negative", |c| {
        c.iter(|| black_box(X).is_negative())
    });
    c.bench_function("i1024_zero_is_positive", |c| {
        c.iter(|| black_box(i1024::ZERO).is_positive())
    });
    c.bench_function("i1024_zero_is_negative", |c| {
        c.iter(|| black_box(i1024::ZERO).is_positive())
    });
    c.bench_function("i1024_cmp_pos", |c| {
        c.iter(|| black_box(X).cmp(black_box(&X)))
    });
    c.bench_function("i1024_cmp_mixed", |c| {
        c.iter(|| black_box(X).cmp(black_box(&Z)))
    });
    c.bench_function("i1024_cmp_eq", |c| {
        c.iter(|| black_box(X).cmp(black_box(&X)))
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
    criterion_i1024,
    criterion_u1024,
    criterion_posit32,
    criterion_softposit32
);
criterion_main!(benches);
