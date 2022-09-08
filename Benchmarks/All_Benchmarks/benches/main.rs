use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use std::time::Duration;

use desim_benchmark::simulation as desim_simulation;
use sim_v2::simulation as sim_v2_simulation;
use simrs_benchmark::simulation as simrs_simulation;
use simulator_benchmark::simulation as simulator_simulation;

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("simulations");
    for limit in [10000, 20000, 30000, 40000, 50000] {
        group.bench_with_input(BenchmarkId::new("desim", limit), &limit, |b, &limit| {
            b.iter(|| desim_simulation(black_box(limit as f64)));
        });

        group.bench_with_input(BenchmarkId::new("simrs", limit), &limit, |b, &limit| {
            b.iter(|| simrs_simulation(black_box(limit)));
        });

        group.bench_with_input(BenchmarkId::new("simulator", limit), &limit, |b, &limit| {
            b.iter(|| simulator_simulation(black_box(limit)));
        });

        group.bench_with_input(
            BenchmarkId::new("simulator_v2", limit),
            &limit,
            |b, &limit| {
                b.iter(|| sim_v2_simulation(black_box(limit)));
            },
        );
    }
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(100));
    targets = bench
);
criterion_main!(benches);
