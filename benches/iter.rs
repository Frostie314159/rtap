use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rtap::create_radiotap_iterator;
const TEST_BYTES: [u8; 32] = [
    0x00, 0x00, 0x20, 0x00, 0xae, 0x40, 0x00, 0xa0, 0x20, 0x08, 0x00, 0xa0, 0x20, 0x08, 0x00, 0x00,
    0x10, 0x18, 0x64, 0x14, 0x40, 0x01, 0xaa, 0x00, 0x21, 0x00, 0x00, 0x00, 0xac, 0x00, 0xa8, 0x01,
];
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("iter", |b| {
        b.iter(|| {
            create_radiotap_iterator(black_box(&mut TEST_BYTES.iter().copied()))
                .unwrap()
                .count()
        })
    });
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
