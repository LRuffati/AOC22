use std::io::BufRead;
use crate::days::Day;
use regex::Regex;

#[derive(Debug)]
pub struct Day5{
    cols: Vec<Vec<u8>>,
    moves: Vec<(usize, usize, usize)>,
}

impl Day for Day5 {
    const DAY: usize = 5;

    fn create<B: BufRead>(input: B) -> Self {
        let mut itl = input.lines().map(|x| x.unwrap());
        let mut vec_vecs = Vec::new();
        for s in itl.by_ref().take_while(|x| !x.starts_with(" 1")){
            //println!("{}", &s);
            for (co, ch) in s.as_bytes().iter().skip(1).step_by(4).enumerate(){
                if co >= vec_vecs.len(){ vec_vecs.push(Vec::new())}
                if *ch != b' ' {vec_vecs[co].push(*ch)}
            }
        }
        let mut vec_rls = Vec::new();
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        for s in itl.skip(1) {
            let ca = re.captures(s.as_str()).unwrap();
            vec_rls.push((ca[1].parse::<usize>().unwrap(), ca[2].parse::<usize>().unwrap(), ca[3].parse::<usize>().unwrap()))
        }
        Day5{cols: vec_vecs.iter().map(|x| x.into_iter().rev().map(|x| *x).collect()).collect(), moves: vec_rls}
    }

    fn solve_a(mut self) {
        for (n, f, t) in self.moves{
            for _ in 0..n {
                if let Some(x) = self.cols[f-1].pop() {
                    self.cols[t-1].push(x);
                }
            }
        }
        println!("{}", std::str::from_utf8(&self.cols.iter().map(|x| *(x.last().unwrap())).collect::<Vec<u8>>()).unwrap());
    }

    fn solve_b(mut self) {
        for (n, f, t) in self.moves{
            let mut v = Vec::new();
            for _ in 0..n {
                if let Some(x) = self.cols[f-1].pop() {
                    v.push(x);
                } else {
                    break;
                }
            }

            let mut x = v.into_iter().rev().collect();
            self.cols[t-1].append(&mut x);
        }
        println!("{}", std::str::from_utf8(&self.cols.iter().map(|x| *(x.last().unwrap())).collect::<Vec<u8>>()).unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn test_a(){
        let d =  Day5::create(INPUT.as_bytes());
        d.solve_a();
    }

    #[test]
    fn test_b(){
        let d =  Day5::create(INPUT.as_bytes());
        d.solve_b();
    }
}