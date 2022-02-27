use criterion::{criterion_group, criterion_main, Criterion};
use pci_id::pci_ids::{PciIds, PATH_TO_PCI_IDS};
use std::path::Path;

pub fn bench_parse_all(c: &mut Criterion) {
    c.bench_function("parse all", |b| {
        b.iter(|| {
            let _ = PciIds::new();
            PciIds::parse_pci_id_list(Path::new(PATH_TO_PCI_IDS))
                .unwrap();
        })
    });
}

pub fn bench_parse_vendors(c: &mut Criterion) {
    c.bench_function("parse only vendors", |b| {
        b.iter(|| {
            let mut pci_ids = PciIds::new();
            pci_ids
                .parse_vendors(Path::new(PATH_TO_PCI_IDS))
                .unwrap();
        })
    });
}

pub fn bench_parse_classes(c: &mut Criterion) {
    c.bench_function("parse only classes", |b| {
        b.iter(|| {
            let mut pci_ids = PciIds::new();
            pci_ids
                .parse_classes(Path::new(PATH_TO_PCI_IDS))
                .unwrap();
        })
    });
}
criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = bench_parse_all, bench_parse_vendors, bench_parse_classes
}
criterion_main!(benches);
