use std::io::BufRead;
use crate::days::Day;

struct Acc {
    max_id: Option<usize>,
    max_cal: usize,
    curr_id: usize,
    curr_cal: usize,
}

impl Default for Acc {
    fn default() -> Self {
        Acc{max_id: None, max_cal: 0, curr_id: 0, curr_cal: 0}
    }
}

impl Acc {
    fn step(mut self, val: Option<usize>) -> Self {
        match val {
            None => {
                self.curr_id +=1;
                self.curr_cal = 0;
                self
            },
            Some(x) => {
                self.curr_cal += x;
                if self.curr_cal >= self.max_cal {
                    self.max_id = Some(self.curr_id);
                    self.max_cal = self.curr_cal;
                }
                self
            }
        }
    }
}

pub struct Day1<'a>(
    Box<dyn Iterator<Item=Option<usize>> + 'a>
);

impl<'a> Day for Day1<'a> {
    const DAY: usize = 1;

    fn create<B:'a + BufRead>(input: B) -> Self {
        Day1(Box::new(input.lines().map(|num| num.unwrap().parse::<usize>().ok())))
    }

    fn solve_a(self) {
        let res: Acc = self.0.fold(Acc::default(), |a, e| a.step(e));
        println!("{}", res.max_cal);
    }

    fn solve_b(self) {
        let mut res = self.0.fold((Vec::new(), 0usize), |mut a, e|
            match e {
                Some(x) => {a.1 += x; a},
                None => {a.0.push((a.0.len(), a.1)); (a.0, 0)}
            }).0;
        res.sort_by_key(|e| e.1);
        res.reverse();
        let ret = res.iter().take(3).map(|e|e.1).reduce(|a, e| a+e);
        println!("{}", ret.unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

    #[test]
    fn test_a(){
        let d =  Day1::create(INPUT.as_bytes());
        d.solve_a();
    }

    #[test]
    fn test_b(){
        let d =  Day1::create(INPUT.as_bytes());
        d.solve_b();
    }
}