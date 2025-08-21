use aoc_2025_lib::day11::{blink_brute, blink_mapped};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn bench_blinks(c: &mut Criterion) {
    let stones = vec![125, 17];

    let mut group = c.benchmark_group("blinks");

    for blinks in [1, 10, 20] {
        group.bench_with_input(BenchmarkId::new("brute", blinks), &blinks, |b, &blinks| {
            b.iter(|| blink_brute(&stones, blinks));
        });
        group.bench_with_input(BenchmarkId::new("mapped", blinks), &blinks, |b, &blinks| {
            b.iter(|| blink_mapped(&stones, blinks));
        });
    }

    group.finish();
}

criterion_group!(benches, bench_blinks);
criterion_main!(benches);
