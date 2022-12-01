mod day01;

use std::fs::File;
use std::io::BufReader;

pub trait Day {
    fn create(input: BufReader<File>) -> Self;

    fn solve_a(self);

    fn solve_b(self);
}

pub type Curr = day01::Day1;