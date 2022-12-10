use std::io::BufRead;
use regex::{Match, Regex};

pub struct Day(Vec<Instr>, usize, i32);

struct Instr {
    val_beg: i32,
    clock_beg: usize,
    change: Option<i32>,
}

impl super::Day for Day {
    const DAY: usize = 10;

    fn create<B: BufRead>(input: B) -> Self {
        let re = Regex::new(r"(?P<noop>noop)|add(.) (?P<chg>-?\d+)").unwrap();
        let mut clock = 1;
        let mut val = 1;
        let mut ret = Vec::new();
        for l in input.lines(){
            let l = l.unwrap();
            let c = re.captures(l.as_str()).unwrap();
            let ins = match c.name("chg") {
                None => {
                    let i = Instr{
                        val_beg: val,
                        clock_beg: clock,
                        change: None,
                    };
                    clock += 1;
                    i
                }
                Some(v) => {
                    let c = v.as_str().parse().unwrap();
                    let i = Instr {
                        val_beg: val,
                        clock_beg: clock,
                        change: Some(c),
                    };
                    clock += 2;
                    val += c;
                    i
                }
            };
            ret.push(ins);
        }
        return Day(ret, clock, val);
    }

    fn solve_a(mut self) {
        let mut acc = 0;
        let f_cl = self.1;
        for i in (20..=f_cl).step_by(40) {
            let a = acc;
            if i == f_cl {
                acc += self.2;
                continue;
            }
            match self.0.binary_search_by(|x| x.clock_beg.cmp(&i)) {
                Ok(x) => {acc += i as i32 * self.0.get(x).unwrap().val_beg}
                Err(x) => {acc += i as i32 * self.0.get(x-1).unwrap().val_beg}
            }
            println!("{}: {}", i, acc-a);
        }
        println!("{acc}")
    }

    fn solve_b(mut self) {
        for i in self.0 {
            if i.clock_beg % 40 == 1 {
                println!();
            }
            let mut c = i.clock_beg;
            let mut p = ((c-1) % 40) as i32;
            match i.change {
                None => {if i.val_beg.abs_diff(p) <=1 {print!("#")} else { print!(" ") }}
                Some(_) => {
                    if i.val_beg.abs_diff(p) <=1 {print!("#")} else { print!(" ") }
                    c += 1;
                    p += 1;
                    if c % 40 == 1 { println!(); p=0; }
                    if i.val_beg.abs_diff(p) <=1 {print!("#")} else { print!(" ") }
                }
            }
        }
        println!("")
    }
}

#[cfg(test)]
mod tests {
    use crate::days::Day;
    static INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

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