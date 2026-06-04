use criterion::{Criterion, criterion_group, criterion_main};

use resplatted::client::network::packet_reader::decompress_payload;
use resplatted::client::network::packet_writer::compress_payload;

/// Create a fake compress packet with a define size
/// Use a compression threshold as 256
fn generate_fake_compressed_packet(size: usize, out_buf: &mut Vec<u8>) -> std::io::Result<()> {
    let original_data = vec![0xAA; size];

    compress_payload(&original_data, Some(256), out_buf)
}

fn bench_decompression(c: &mut Criterion) {
    let mut raw_buffer: Vec<u8> = Vec::new();
    generate_fake_compressed_packet(8192, &mut raw_buffer).unwrap();

    let mut out_buffer: Vec<u8> = Vec::new();

    c.bench_function("decompress_payload_8kb", |b| {
        b.iter(|| {
            decompress_payload(
                std::hint::black_box(&raw_buffer),
                std::hint::black_box(Some(256)),
                std::hint::black_box(&mut out_buffer),
            )
            .unwrap();
        })
    });
}

criterion_group!(benches, bench_decompression);
criterion_main!(benches);
