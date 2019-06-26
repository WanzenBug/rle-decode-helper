use criterion::{
    BatchSize,
    black_box,
    Criterion,
    criterion_group,
    criterion_main,
};

use rle_decode_helper;

fn naive(buf: &mut Vec<u8>, fragment: usize, length: usize) {
     for i in 0..length {
        let val = buf[buf.len() - fragment];
        buf.push(val);
     }
}

fn vulnerable(buf: &mut Vec<u8>, fragment: usize, length: usize) {
    
}

fn optimized(buf: &mut Vec<u8>, fragment: usize, length: usize) {
    rle_decode_helper::rle_decode(buf, fragment, length);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut initial = Vec::new();
    for i in 0..=255u8 {
        initial.push(i);
    }
    
    let naive_buf = initial.clone();
    let vulnerable_buf = initial.clone();
    let optimized_buf = initial.clone();
    
    c
        .bench_function(
            "naive",
            move |b| {
                let naive_buf = naive_buf.clone();
                b.iter_batched(
                    move || naive_buf.clone(),
                    |mut buf| naive(black_box(&mut buf), black_box(200), black_box(10_000)),
                    BatchSize::LargeInput,
                )
            }
        )
        .bench_function(
            "vulnerable",
            move |b| {
                let vulnerable_buf = vulnerable_buf.clone();
                b.iter_batched(
                    move || vulnerable_buf.clone(),
                    |mut buf| vulnerable(black_box(&mut buf), black_box(200), black_box(10_000)),
                    BatchSize::LargeInput,
                )
            }
        )
        .bench_function(
            "optimized",
            move |b| {
                let optimized_buf = optimized_buf.clone();
                b.iter_batched(
                    move || optimized_buf.clone(),
                    |mut buf| optimized(black_box(&mut buf), black_box(200), black_box(10_000)),
                    BatchSize::LargeInput,
                )
            }
        );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);