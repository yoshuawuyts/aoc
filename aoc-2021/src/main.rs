#![feature(array_windows)]
#![feature(array_from_fn)]
#![feature(slice_group_by)]

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub(crate) mod stdx;

fn main() {
    day1::run();
    day2::run();
    day3::run();
    day4::run();
    day5::run();
    day6::run();
    day7::run();
    day8::run();
    day9::run();
    day10::run();
    day11::run();
    day12::run();
    day13::run();
    day14::run();
}
