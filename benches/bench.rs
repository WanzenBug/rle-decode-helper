use criterion::{BatchSize, Bencher, black_box, Criterion, criterion_group, criterion_main, ParameterizedBenchmark};

use rle_decode_helper;
use std::fmt::{Debug, Formatter, Error};

#[derive(Clone)]
struct Inputs {
    buffer: Vec<u8>,
    fragment: usize,
    length: usize,
}

impl Debug for Inputs {
    fn fmt<'a>(&self, f: &mut Formatter<'a>) -> Result<(), Error> {
        f.debug_struct("Inputs")
            .field("buffer", &format!("Bytes(length={})", self.buffer.len()))
            .field("fragment", &self.fragment)
            .field("length", &self.length)
            .finish()
    }
}

fn naive(bencher: &mut Bencher, inputs: &Inputs) {
    bencher.iter_batched(
        move || inputs.clone(),
        move |input| {
            let mut input = black_box(input);
            for _ in 0..input.length {
                let val = input.buffer[input.buffer.len() - input.fragment];
                input.buffer.push(val);
            }
            input
        },
        BatchSize::SmallInput,
    )
}

fn vulnerable(bencher: &mut Bencher, inputs: &Inputs) {
    bencher.iter_batched(
        move || inputs.clone(),
        move |input| {
            let mut input = black_box(input);
            input.buffer.reserve(input.length); // allocate required memory immediately, it's faster this way
            unsafe {
                // set length of the buffer up front so we can set elements in a slice instead of pushing
                let len = input.buffer.len();
                input.buffer.set_len(len + input.length);
            }
            for i in (input.buffer.len() - input.length)..input.buffer.len() {
                unsafe {
                    let cpy = *input.buffer.get_unchecked(i - input.fragment);
                    *input.buffer.get_unchecked_mut(i) = cpy;
                }
            }
            input
        },
        BatchSize::SmallInput,
    )
}

fn lib(bencher: &mut Bencher, inputs: &Inputs) {
    bencher.iter_batched(
        move || inputs.clone(),
        move |input| {
            let mut input = black_box(input);
            rle_decode_helper::rle_decode(&mut input.buffer, input.fragment, input.length);
            input
        },
        BatchSize::SmallInput,
    )
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut initial = Vec::new();
    for i in 0..500 {
        initial.push((i & 255) as u8);
    }

    let inputs = vec![
        Inputs {
            buffer: initial.clone(),
            fragment: 333,
            length: 10_000,
        },
        Inputs {
            buffer: initial.clone(),
            fragment: 2,
            length: 10_000,
        },
    ];

    c.bench("rle-impl",
        ParameterizedBenchmark::new("naive", naive, inputs)
            .with_function("vulnerable", vulnerable)
            .with_function("lib", lib)
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
