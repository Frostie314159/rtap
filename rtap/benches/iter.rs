use criterion::{criterion_group, criterion_main, Criterion};
use rtap::frame::RadiotapFrame;
use scroll::Pread;
const TEST_BYTES: &'static [u8] = &[
    0x00, 0x00, 0x20, 0x00, 0xae, 0x40, 0x00, 0xa0, 0x20, 0x08, 0x00, 0xa0, 0x20, 0x08, 0x00, 0x00,
    0x10, 0x18, 0x64, 0x14, 0x40, 0x01, 0xaa, 0x00, 0x21, 0x00, 0x00, 0x00, 0xac, 0x00, 0xa8, 0x01,
];
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("iter", |b| {
        let frame = TEST_BYTES.pread::<RadiotapFrame>(0).unwrap();
        b.iter(|| frame.get_field_iter().count())
    });
    c.bench_function("parse", |b| {
        b.iter(|| {
            let _ = TEST_BYTES.pread::<RadiotapFrame>(0).unwrap();
        })
    });
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
