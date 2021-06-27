use aoc_2020::AoCDay;
use aoc_2020::SinglePart;
use criterion::black_box;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn ninteen01(c: &mut Criterion) {
	use aoc_2020::nineteen01::Code;
	let code: Code = Code{};
    c.bench_function("2019-01 Part 1", |b| b.iter(|| code.part1()));
	c.bench_function("2019-01 Part 2", |b| b.iter(|| code.part2()));
}

pub fn ninteen02(c: &mut Criterion) {
	use aoc_2020::nineteen02::Code;
	let code: Code = Code{};
    c.bench_function("2019-02 Part 1", |b| b.iter(|| code.part1()));
	c.bench_function("2019-02 Part 2", |b| b.iter(|| code.part2()));
}

pub fn ninteen03(c: &mut Criterion) {
	use aoc_2020::nineteen03::Code;
	let code: Code = Code{};
    c.bench_function("2019-03 Only Part", |b| b.iter(|| code.run()));
}

pub fn ninteen04(c: &mut Criterion) {
	use aoc_2020::nineteen04::count_passwords_between;
    c.bench_function("2019-04 Action function", |b| b.iter(|| count_passwords_between(black_box(168630), black_box(718098))));
}

pub fn ninteen05(c: &mut Criterion) {
	use aoc_2020::nineteen05::{Code};
	let code: Code = Code{};
	c.bench_function("2019-05 Part 1", |b| b.iter(|| code.part1()));
	c.bench_function("2019-05 Part 2", |b| b.iter(|| code.part2()));
}

pub fn twenty22(c: &mut Criterion) {
	use aoc_2020::day22::{Code};
	let code: Code = Code{};
	c.bench_function("2020-22 Part 1", |b| b.iter(|| code.part1()));
	c.bench_function("2020-22 Part 2", |b| b.iter(|| code.part2()));
}

pub fn twenty23(c: &mut Criterion) {
	use aoc_2020::day23::{Code};
	let code: Code = Code{};
	c.bench_function("2020-23 Part 1", |b| b.iter(|| code.part1()));
	c.bench_function("2020-23 Part 2", |b| b.iter(|| code.part2()));
}

pub fn twenty24(c: &mut Criterion) {
	use aoc_2020::day24::{Code};
	let code: Code = Code{};
	c.bench_function("2020-24 Part 1", |b| b.iter(|| code.part1()));
	c.bench_function("2020-24 Part 2", |b| b.iter(|| code.part2()));
}

pub fn twenty25(c: &mut Criterion) {
	use aoc_2020::day25::Code;
	let code: Code = Code{};
    c.bench_function("2020-25 Only Part", |b| b.iter(|| code.run()));
}

criterion_group!(benches, ninteen01, ninteen02, ninteen03, ninteen04, ninteen05, twenty22, twenty23, twenty24, twenty25);
criterion_main!(benches);