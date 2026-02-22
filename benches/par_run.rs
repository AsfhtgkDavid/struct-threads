use criterion::{Criterion, criterion_group, criterion_main};
use struct_threads::{ParallelRun, Runnable};

struct TestTask(u64);

impl Runnable for TestTask {
    type Output = u64;

    fn run(self) -> Self::Output {
        let mut acc = 0u64;
        for i in 0..2000 {
            acc = acc.wrapping_add((i ^ self.0).wrapping_mul(997));
        }
        acc
    }
}

fn bench_sequential(c: &mut Criterion) {
    let input: Vec<u64> = (0..5000).collect();

    c.bench_function("sequential", |b| {
        b.iter(|| {
            input
                .iter()
                .map(|&x| {
                    let mut acc = 0u64;
                    for i in 0..2000 {
                        acc = acc.wrapping_add((i ^ x).wrapping_mul(997));
                    }
                    acc
                })
                .collect::<Vec<_>>()
        })
    });
}

fn bench_par_run(c: &mut Criterion) {
    let input: Vec<u64> = (0..5000).collect();

    c.bench_function("par_run", |b| {
        b.iter(|| {
            input
                .iter()
                .map(|&x| TestTask(x))
                .collect::<Vec<_>>()
                .par_run()
                .unwrap()
        })
    });
}

fn benches(c: &mut Criterion) {
    bench_sequential(c);
    bench_par_run(c);
}

criterion_group!(benches_group, benches);
criterion_main!(benches_group);
