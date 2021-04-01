use criterion::{black_box, criterion_group, criterion_main, Criterion};

use flo_curves::geo::*;
use rand::prelude::*;
use std::cmp::{Ordering};

fn sweep(n: usize) {
    let mut rng     = StdRng::from_seed([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31]);
    let mut bounds  = (0..n).into_iter()
        .map(|_| {
            let x = rng.gen::<f64>() * 900.0;
            let y = rng.gen::<f64>() * 900.0;
            let w = rng.gen::<f64>() * 400.0;
            let h = rng.gen::<f64>() * 400.0;

            Bounds::from_min_max(Coord2(x, y), Coord2(x+w, y+h))
        })
        .collect::<Vec<_>>();
    bounds.sort_by(|b1, b2| b1.min().x().partial_cmp(&b2.min().x()).unwrap_or(Ordering::Equal));

    let _ = sweep_self(bounds.iter()).collect::<Vec<_>>();
}

fn sweep_slow(n: usize) {
    let mut rng     = StdRng::from_seed([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31]);
    let bounds      = (0..n).into_iter()
        .map(|_| {
            let x = rng.gen::<f64>() * 900.0;
            let y = rng.gen::<f64>() * 900.0;
            let w = rng.gen::<f64>() * 400.0;
            let h = rng.gen::<f64>() * 400.0;

            Bounds::from_min_max(Coord2(x, y), Coord2(x+w, y+h))
        })
        .collect::<Vec<_>>();

    let mut slow_collisions = vec![];

    for i1 in 0..bounds.len() {
        for i2 in 0..i1 {
            if i1 == i2 { continue; }

            if bounds[i1].overlaps(&bounds[i2]) {
                slow_collisions.push((&bounds[i1], &bounds[i2]));
            }
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sweep 10", |b| b.iter(|| sweep(black_box(10))));
    c.bench_function("sweep_slow 10", |b| b.iter(|| sweep_slow(black_box(10))));

    c.bench_function("sweep 100", |b| b.iter(|| sweep(black_box(100))));
    c.bench_function("sweep_slow 100", |b| b.iter(|| sweep_slow(black_box(100))));

    c.bench_function("sweep 1000", |b| b.iter(|| sweep(black_box(1000))));
    c.bench_function("sweep_slow 1000", |b| b.iter(|| sweep_slow(black_box(1000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
