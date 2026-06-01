/// Use `cargo bench --bench packet_decompression -- --save-baseline main` to save
/// After the change, adapt and run `cargo bench --bench packet_decompression -- --baseline main`
use bytes::BytesMut;
use criterion::{Criterion, criterion_group, criterion_main};

use resplatted::client::network::packet_reader::decompress_payload;
use resplatted::client::network::packet_writer::compress_payload;

/// Create a fake compress packet with a define size
/// Use a compression threshold as 256
fn generate_fake_compressed_packet(size: usize) -> BytesMut {
    let original_data = vec![0xAA; size];

    compress_payload(&original_data, Some(256)).unwrap()
}

fn bench_decompression(c: &mut Criterion) {
    let network_packet = generate_fake_compressed_packet(8192);
    let mut out_buffer: Vec<u8> = Vec::new();

    c.bench_function("decompress_payload_8kb", |b| {
        b.iter(|| {
            let _result = decompress_payload(
                std::hint::black_box(&network_packet),
                std::hint::black_box(Some(256)),
                std::hint::black_box(&mut out_buffer),
            )
            .unwrap();
        })
    });
}

criterion_group!(benches, bench_decompression);
criterion_main!(benches);
