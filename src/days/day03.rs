use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::days::Day;

pub struct Day3(
    Box<dyn Iterator<Item=(Vec<u8>, Vec<u8>)>>
);

fn to_list(s: String) -> Vec<u8>{
    s.into_bytes().into_iter().map(|c| {if c>=97 {c-96} else {c-64+26}}).collect()
}

impl Day for Day3 {
    const DAY: usize = 3;

    fn create(input: BufReader<File>) -> Self {
        let a = input.lines().map(|ln|{
            let mut fs = ln.unwrap();
            let sn = fs.split_off(fs.len()/2);
            //println!("{}, {}", fs, sn);
            (to_list(fs), to_list(sn))
        });
        Day3(Box::new(a))
    }

    fn solve_a(self) {
        println!("{}", self.0.map(|(f, s)| {
            let s1 = BTreeSet::from_iter(f.into_iter());
            let s2 = BTreeSet::from_iter(s.into_iter());
            //println!("{:?} {:?}", s1, s2);
            s1.intersection(&s2).into_iter().fold(0, |a, b| {a+(*b as usize)})
        }).fold(0, |a, b| a+b));
    }

    fn solve_b(self) {
        println!("{}",
            self.0.fold((0usize, 0u8, None::<BTreeSet<u8>>), |mut a, (f, s)|{
                let mut fs = BTreeSet::from_iter(f.into_iter());
                let mut ss = BTreeSet::from_iter(s.into_iter());
                fs.append(&mut ss);
                a.2 = Some(match a.2{
                    None => fs,
                    Some(s) => BTreeSet::from_iter(s.intersection(&fs).into_iter().map(|a| *a)),
                });
                a.1 += 1;
                if a.1 == 3 {
                    if let Some(s) = a.2 {
                        a.0 += s.into_iter().fold(0, |a, b| a + (b as usize));
                        a.1 = 0;
                        a.2 = None;
                    }
                }
                a
            }).0);
    }
}