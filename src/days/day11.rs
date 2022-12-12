use std::io::BufRead;
use std::str::FromStr;
use regex::{Regex, RegexBuilder};
use crate::days::day11::Operand::{Old, Val};
use crate::days::day11::Operation::{Add, Mul};

#[derive(Debug)]
pub struct Day(Vec<Monkey>);

#[derive(Debug, Copy, Clone)]
enum Operand {
    Old,
    Val(isize)
}

impl Operand {
    fn get_val(&self, old: isize) -> isize {
        match self {
            Operand::Old => {old}
            Operand::Val(x) => {*x}
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add(Operand, Operand),
    Mul(Operand, Operand)
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"\s*(old|(?P<l1>\d+))\s*((?P<m>\*)|(?P<s>\+))\s+(old|(?P<l2>\d+))").unwrap();
        let m = re.captures(s).ok_or(())?;
        let op1 = match m.name("l1") {
            None => {Old}
            Some(v) => {Val(v.as_str().parse().unwrap())}
        };
        let op2 = match m.name("l2") {
            None => {Old}
            Some(v) => {Val(v.as_str().parse().unwrap())}
        };

        if let Some(_) = m.name("m") {return Ok(Mul(op1, op2))}
        if let Some(_) = m.name("s") {return Ok(Add(op1, op2))}
        Err(())
    }
}

impl Operation {
    fn res(&self, old: isize) -> isize {
        match self {
            Operation::Add(x, y) => {x.get_val(old) + y.get_val(old)}
            Operation::Mul(x, y) => {x.get_val(old) * y.get_val(old)}
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<isize>,
    div: isize,
    op: Operation,
    dest: [usize; 2],
    ops: usize,

}

impl Monkey {
    fn inspect_all(&mut self) -> Vec<(usize, isize)> {
        let mut c = 0;
        let mut v = Vec::new();
        for i in self.items.drain(..){
            c+=1;
            let r = self.op.res(i);
            let r = r/3;
            if r % self.div == 0 {
                v.push((self.dest[0], r));
            } else {
                v.push((self.dest[1], r));
            }
        }
        self.ops += c;
        v
    }

    fn inspect_b(&mut self, div:isize) -> Vec<(usize, isize)>{
        let mut c = 0;
        let mut v = Vec::new();
        for i in self.items.drain(..){
            c+=1;
            let r = self.op.res(i);
            let r = r%div;
            if r % self.div == 0 {
                v.push((self.dest[0], r));
            } else {
                v.push((self.dest[1], r));
            }
        }
        self.ops += c;
        v
    }
}

impl super::Day for Day {
    const DAY: usize = 11;

    fn create<B: BufRead>(mut input: B) -> Self {
        let mut re = RegexBuilder::new(r"Monkey (?P<id>\d+):\s+.+: (?P<list>\d+(, \d+)*)\s+.+=(?P<ops>.+)\s+.+: (divisible) by (?P<div>\d+)\s+.+(?P<dtr>\d+)\s+.+(?P<dfl>\d+)\n*");
        re.multi_line(true);
        let re = re.build().unwrap();
        let mut str = String::new();
        if let Err(_) = input.read_to_string(&mut str) {panic!()};

        let mut monkeys: Vec<Monkey> = Vec::new();
        for m in re.captures_iter(str.as_str()) {
            let items = m.name("list").unwrap().as_str().split(", ").map(|x| x.parse().unwrap()).collect();
            let op= m.name("ops").unwrap().as_str().parse().unwrap();
            let div = m.name("div").unwrap().as_str().parse().unwrap();
            let dtr = m.name("dtr").unwrap().as_str().parse().unwrap();
            let dfl = m.name("dfl").unwrap().as_str().parse().unwrap();
            monkeys.push(Monkey{items, div, op, dest: [dtr, dfl], ops: 0})
        }

        return Day(monkeys);
    }

    fn solve_a(mut self) {
        for _i in 0..20 {
            for m in 0..self.0.len() {
                let dests = self.0.get_mut(m).unwrap().inspect_all();
                for (d, v) in dests.into_iter() {
                    self.0.get_mut(d).unwrap().items.push(v);
                }
            }
        }

        let mut vals: Vec<usize> = self.0.iter().map(|x|(x.ops)).collect();
        vals.sort();
        vals.reverse();
        println!("{}", vals.into_iter().take(2).reduce(|a, x| a*x).unwrap() );
    }

    fn solve_b(mut self) {
        let mut d = 1;
        for m in self.0.iter() {
            d *= m.div;
        }
        for _i in 0..10000 {
            for m in 0..self.0.len() {
                let dests = self.0.get_mut(m).unwrap().inspect_b(d);
                for (d, v) in dests.into_iter() {
                    self.0.get_mut(d).unwrap().items.push(v);
                }
            }
        }

        let mut vals: Vec<usize> = self.0.iter().map(|x|(x.ops)).collect();
        vals.sort();
        vals.reverse();
        println!("{}", vals.into_iter().take(2).reduce(|a, x| a*x).unwrap() );
    }
}

#[cfg(test)]
mod tests {
    use crate::days::Day;
    static INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";
    #[test]
    fn test_parse() {
        let d = super::Day::create(INPUT.as_bytes());
        println!("{:?}", d);
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
