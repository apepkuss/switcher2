use criterion::{black_box, criterion_group, criterion_main, Criterion};

use switcher2::stack::*;
use switcher2::Generator;

fn switcher2(c: &mut Criterion) {
    // Bench allocation
    c.bench_function("create 8 MB stack", |b| b.iter(|| EightMbStack::new()));

    c.bench_function("switch stacks", |b| {
        let stack = EightMbStack::new().unwrap();
        let mut gen = Generator::new(stack, |yielder, input| {
            black_box(yielder.suspend(input + 1));
        });
        b.iter(|| black_box(gen.resume(2)))
    });
}

criterion_group!(benches, switcher2);
criterion_main!(benches);
