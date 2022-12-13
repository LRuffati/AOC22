use std::{io::BufRead, iter::zip};

use itertools::Itertools;
use nom::{IResult, sequence::{delimited, preceded}, bytes::complete::tag, multi::many0, combinator::{map, opt}, branch::alt};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Elem {
    Int(u64),
    Lst(Vec<Self>),
}

#[derive(Debug)]
enum ResCom{
    Order,
    NotOrder,
    Continue
}

impl Elem {
    fn in_order(fst: &Self, snd: &Self) -> ResCom {
        match (fst, snd) {
            (Elem::Int(x), Elem::Int(y)) => {
                if x < y {ResCom::Order}
                else if x==y {ResCom::Continue}
                else {ResCom::NotOrder}
            },
            (Elem::Int(x), Elem::Lst(_)) => {
                Self::in_order(&Elem::Lst(vec![Elem::Int(*x)]), snd)
            },
            (Elem::Lst(_), Elem::Int(x)) => {
                Self::in_order(fst, &Elem::Lst(vec![Elem::Int(*x)]))
            },
            (Elem::Lst(v), Elem::Lst(l)) => {
                let mut r = ResCom::Continue;
                for (f, s) in zip(v.iter(), l.iter()) {
                    r = Elem::in_order(f, s);
                    match r {
                        ResCom::Order => {break;},
                        ResCom::NotOrder => {break;},
                        ResCom::Continue => {},
                    };
                }
                if let ResCom::Continue = &r {
                    r = match v.len().cmp(&l.len()) {
                        std::cmp::Ordering::Less => ResCom::Order,
                        std::cmp::Ordering::Equal => ResCom::Continue,
                        std::cmp::Ordering::Greater => ResCom::NotOrder,
                    }
                }
                r
            },
        }
    }
}

impl PartialOrd<Elem> for Elem {
    fn partial_cmp(&self, other: &Elem) -> Option<std::cmp::Ordering> {
        Some(match Elem::in_order(self, other) {
            ResCom::Order => std::cmp::Ordering::Less,
            ResCom::NotOrder => std::cmp::Ordering::Greater,
            ResCom::Continue => std::cmp::Ordering::Equal,
        })
    }
}

impl Ord for Elem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse_elements(i: &str) -> IResult<&str, Elem> {
    let lst = map(delimited(tag("["), many0(preceded(opt(tag(",")), parse_elements)), tag("]")), Elem::Lst);
    let int = map(nom::character::complete::u64, Elem::Int);
    alt((lst, int))(i)
}

#[derive(Debug)]
pub struct Day(Vec<(Elem, Elem)>);

impl super::Day for Day {
    const DAY: usize =13;

    fn create<B: BufRead>(input: B) -> Self {
        let it = input.lines().filter(|x| {
            if let Ok(s) = x {
                !s.is_empty()
            } else {
                false
            }
        });
        let mut v = Vec::new();

        for (fs, ss) in it.tuples(){
            let fs = fs.unwrap();
            let ss = ss.unwrap();
            let (f_r, f) = parse_elements(fs.as_str().clone()).unwrap();
            let (s_r, s) = parse_elements(ss.as_str().clone()).unwrap();

            if f_r == s_r && f_r == "" {v.push((f, s))} else {panic!()}
        }
        return Day(v);
    }

    fn solve_a(self) {
        let mut r = 0;
        for (idx, (fs, sn)) in self.0.into_iter().enumerate() {
            match Elem::in_order(&fs, &sn) {
                ResCom::Order => {r += idx+1},
                ResCom::NotOrder => {},
                ResCom::Continue => {
                    panic!();
                },
            }
        }
        println!("{r}");
    }

    fn solve_b(self) {
        let mut v = Vec::new();

        for (a, b) in self.0 {
            v.push(a);
            v.push(b);
        }
        let a = Elem::Lst(vec![Elem::Lst(vec![Elem::Int(2)])]);
        v.push(a.clone());
        let b = Elem::Lst(vec![Elem::Lst(vec![Elem::Int(6)])]);
        v.push(b.clone());
        v.sort();

        let pa = v.binary_search(&a).unwrap()+1;
        let pb = v.binary_search(&b).unwrap()+1;
        println!("{},{} => {}", pa, pb, pa*pb);
    }
}

#[cfg(test)]
mod tests {
    use crate::days::Day;

    use super::parse_elements;
    static INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_lst(){
        let s = "[[1],[2,3,4],[[[]]]]";
        let r = parse_elements(s).unwrap();
        println!("{:?}", r);
    }

    #[test]
    fn test_parse() {
        let d = super::Day::create(INPUT.as_bytes());
        for i in d.0 {
            println!("{:?}", i);
        }
    }

    #[test]
    fn test_a(){
        let d =  super::Day::create(INPUT.as_bytes());
        d.solve_a();
    }

    #[test]
    fn test_b(){
        let d =  super::Day::create(INPUT.as_bytes());
        d.solve_b();
    }
}
