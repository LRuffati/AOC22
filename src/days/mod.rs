mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

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

pub type Curr = day10::Day;