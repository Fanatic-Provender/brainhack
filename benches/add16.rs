use criterion::{criterion_group, criterion_main, Criterion};
use brainhack::hackfuck::{Parser, Interpreter};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("16 Bit Add", 
    |b| b.iter(
        || {
            let program = Parser::from_file("test/add16.bf".to_owned())
                .unwrap()
                .optimized_parse();
            let mut interpreter = Interpreter::new(program);
            interpreter.eval().unwrap();
        }
    ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);