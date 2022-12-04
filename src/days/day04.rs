use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use crate::days::Day;

pub struct Day4(
    Box<dyn Iterator<Item=Vec<Assignment>>>
);

struct Assignment(usize, usize);

impl FromStr for Assignment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let l: Vec<usize> = s.split('-').map(|a| a.parse::<usize>().unwrap()).collect();
        Ok(Assignment(l[0], l[1]))
    }
}

impl Assignment {
    fn overlaps_full(&self, oth: &Self) -> bool {
        if (self.0 == oth.0) || (self.1 == oth.1) {
            return true;
        }
        return if self.0 < oth.0 {
            self.1 > oth.1
        } else {
            self.1 < oth.1
        }
    }
    fn overlaps(&self, oth: &Self) -> bool {
        if (self.0 == oth.0) || (self.1 == oth.1) {
            return true;
        }
        return if self.0 < oth.0 {
            self.1 >= oth.0
        } else {
            oth.1 >= self.0
        }

    }
}

impl Day for Day4 {
    const DAY: usize = 4;

    fn create(input: BufReader<File>) -> Self {
        let r = input.lines().map(|l| {
            l.unwrap().split(",").map(|s| s.parse::<Assignment>().unwrap()).collect::<Vec<Assignment>>()
        });
        Day4(Box::new(r))
    }

    fn solve_a(self) {
        let v = self.0.fold(0usize, |a, e| {
            if e[0].overlaps_full(&e[1]){
                a+1
            } else { a }
        });

        println!("{}", v);
    }

    fn solve_b(self) {
        println!("{}", self.0.fold(0usize, |a, e|{
            if e[0].overlaps(&e[1]){
                a+1
            } else { a }
        }));
    }
}