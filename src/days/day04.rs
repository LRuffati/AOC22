use std::io::BufRead;
use std::str::FromStr;
use crate::days::Day;

pub struct Day4<'a>(
    Box<dyn Iterator<Item=Vec<Assignment>> + 'a>
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

impl<'a> Day for Day4<'a> {
    const DAY: usize = 4;

    fn create<B: BufRead + 'a>(input: B) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn test_a(){
        let d =  Day4::create(INPUT.as_bytes());
        d.solve_a();
    }

    #[test]
    fn test_b(){
        let d =  Day4::create(INPUT.as_bytes());
        d.solve_b();
    }
}