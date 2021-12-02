#![feature(array_windows)]
#![feature(array_from_fn)]

mod day1;
mod day2;

pub(crate) mod stdx;

fn main() {
    day1::run();
    day2::run();
}
