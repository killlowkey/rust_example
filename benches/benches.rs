use criterion::{Bencher, Criterion, criterion_group, criterion_main};
use rust_example::abi::business::PersonService;
use rust_example::abi::pb::PbPerson;

fn bench_simple(b: &mut Bencher) {
    let mut service = PersonService::new();
    service.add(PbPerson::new("ray".into(), 20)).expect("TODO: panic message");
    b.iter(|| {
        service.get("ray");
    })
}

fn bench_query(c: &mut Criterion) {
    let mut group = c.benchmark_group("query");
    group.bench_function("simple", bench_simple);
    group.finish();
}

// 执行 cargo bench 运行，由 criterion 库提供支持
// 生成报告：cargo bench --target-dir=benches/report
criterion_group!(bench, bench_query);
criterion_main!(bench);

