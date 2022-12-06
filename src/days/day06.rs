use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::days::Day;

pub struct Day6(Box<dyn Iterator<Item=u8>>);

/// First element the place in which to insert next
#[derive(Debug)]
struct Acc<const N: usize>(usize, [Option<u8>; N]);

impl<const N: usize> Acc<N> {
    fn new() -> Self {
        Acc(0, [None; N])
    }

    fn check_insert(&mut self, c: u8) -> bool {
        self.1[self.0] = Some(c);
        self.0 = (self.0 + 1) % N;
        //println!("{}, {:?}", c, self);

        for i in self.1.as_ref() {
            if i.is_none() {return false;}
            let l = self.1.into_iter().filter(|x| x == i).count();
            if l > 1 {return false;}
        }
        true
    }
}

fn solve<const N: usize>(d: Day6, mut acc: Acc<N>) -> usize{
    let mut fp = None;
    for (p, e) in d.0.enumerate() {
        if acc.check_insert(e) {
            fp = Some(p);
            break;
        }
    }
    return fp.unwrap()+1

}

impl Day for Day6 {
    const DAY: usize = 6;

    fn create(input: BufReader<File>) -> Self {
        Day6(Box::new(input.lines().next().unwrap().unwrap().into_bytes().into_iter()))
    }

    fn solve_a(self) {
        let acc = Acc::<4>::new();
        println!("{}", solve(self, acc));
    }

    fn solve_b(self) {
        let acc = Acc::<14>::new();
        println!("{}", solve(self, acc));
    }
}