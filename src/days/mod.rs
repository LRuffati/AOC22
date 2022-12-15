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
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
//mod day07b;

use std::io::BufRead;

pub trait Day {
    const DAY: usize;

    fn path() -> String {
        format!("inputs/{:02}", Self::DAY)
    }

    fn create<B: BufRead + 'static>(input: B) -> Self;

    fn solve_a(self);

    fn solve_b(self);
}

pub type Curr = day15::Day;