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

pub fn ninteen06(c: &mut Criterion) {
	use aoc_2020::nineteen06::{part_1_impl, part_2_impl, FL_CONT};
	c.bench_function("2019-06 Part 1", |b| b.iter(|| part_1_impl(black_box(FL_CONT))));
	c.bench_function("2019-06 Part 2", |b| b.iter(|| part_2_impl(black_box(FL_CONT))));
}

pub fn ninteen07(c: &mut Criterion) {
	use aoc_2020::nineteen07::{perform_work, DAY_7_DATA};
	let data: Vec<i64> = Vec::from(DAY_7_DATA);
	c.bench_function("2019-07 Part 1", |b| b.iter(|| perform_work(black_box(data.clone()), black_box(0..=4), black_box(false))));
	c.bench_function("2019-07 Part 2", |b| b.iter(|| perform_work(black_box(data.clone()), black_box(5..=9), black_box(true))));
}

pub fn ninteen08(c: &mut Criterion) {
	use aoc_2020::nineteen08::{part_1_impl, part_2_impl, DAY_8_DATA};
	c.bench_function("2019-08 Part 1", |b| b.iter(|| part_1_impl(black_box(DAY_8_DATA), black_box(25), black_box(6))));
	c.bench_function("2019-08 Part 2", |b| b.iter(|| part_2_impl(black_box(DAY_8_DATA), black_box(25), black_box(6))));
}

pub fn ninteen09(c: &mut Criterion) {
	use aoc_2020::nineteen09::{parts_impl, DAY_9_DATA};
	c.bench_function("2019-09 Part 1", |b| b.iter(|| parts_impl(black_box(DAY_9_DATA), black_box(1))));
	c.bench_function("2019-09 Part 2", |b| b.iter(|| parts_impl(black_box(DAY_9_DATA), black_box(9))));
}

pub fn ninteen10(c: &mut Criterion) {
	use aoc_2020::nineteen10::{solve, FL_CONT};
    c.bench_function("2019-10 Action function", |b| b.iter(|| solve(black_box(FL_CONT))));
}

pub fn ninteen11(c: &mut Criterion) {
	use aoc_2020::nineteen11::{part_1_impl, part_2_impl, DAY_11_DATA};
	c.bench_function("2019-11 Part 1", |b| b.iter(|| part_1_impl(black_box(DAY_11_DATA))));
	c.bench_function("2019-11 Part 2", |b| b.iter(|| part_2_impl(black_box(DAY_11_DATA))));
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

criterion_group!(benches, ninteen10, ninteen11, ninteen07,
	 ninteen08, ninteen09, ninteen01, ninteen02, ninteen03, 
	 ninteen04, ninteen05, ninteen06, twenty22, twenty23, 
	 twenty24, twenty25);
criterion_main!(benches);