#![feature(array_windows)]
#![feature(array_from_fn)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

pub(crate) mod stdx;

fn main() {
    day1::run();
    day2::run();
    day3::run();
    day4::run();
    day5::run();
}
