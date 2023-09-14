use criterion::{criterion_group, criterion_main, Criterion};
use std::process::Command;

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("main_function_group");
    group.sample_size(10); // Reduce the number of samples
    group.warm_up_time(std::time::Duration::from_millis(500)); // Reduce the warm-up time

    group.bench_function("main_function", |b| {
        b.iter(|| {
            let output = Command::new("target/release/gdlauncher")
                .output()
                .expect("failed to execute process");
            assert!(output.status.success());
        })
    });

    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
