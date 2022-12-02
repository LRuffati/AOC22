mod day01;
mod day02;

use std::fs::File;
use std::io::BufReader;

pub trait Day {
    const DAY: usize;

    fn path() -> String {
        format!("inputs/{:02}", Self::DAY)
    }

    fn create(input: BufReader<File>) -> Self;

    fn solve_a(self);

    fn solve_b(self);
}

pub type Curr = day02::Day2;