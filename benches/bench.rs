use criterion::{criterion_group, criterion_main, Criterion};
use pci_id::{parse_pci_id_list, DEFAULT_PATH_TO_PCI_IDS};

pub fn bench_parse_list(c: &mut Criterion) {
    c.bench_function("parse list 100", |b| {
        b.iter(|| parse_pci_id_list(DEFAULT_PATH_TO_PCI_IDS))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = bench_parse_list
}
criterion_main!(benches);
