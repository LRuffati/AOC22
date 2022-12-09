use std::io::BufRead;
use std::str::FromStr;
use crate::days::Day;

pub struct Day2<'a>(
    Box<dyn Iterator<Item=(First, Second)>+'a>
);

#[derive(Debug)]
enum First {
    R, P, S
}

impl FromStr for First {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_string();
        match s.trim() {
            "A" => Ok(First::R),
            "B"=> Ok(First::P),
            "C" => Ok(First::S),
            _ => Err(())
        }
    }
}

impl First {
    fn win(&self) -> u8 {
        match &self {
            First::R => {2}
            First::P => {3}
            First::S => {1}
        }
    }

    fn tie(&self) -> u8 {
        match &self {
            First::R => {1}
            First::P => {2}
            First::S => {3}
        }
    }

    fn lose(&self) -> u8 {
        match &self {
            First::R => {3}
            First::P => {1}
            First::S => {2}
        }
    }
}

enum Second {
    X, Y, Z,
}

impl FromStr for Second {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_string();
        match s.trim() {
            "X" => Ok(Second::X),
            "Y"=> Ok(Second::Y),
            "Z" => Ok(Second::Z),
            _ => Err(())
        }
    }
}

impl Second {
    fn result(f: First, s: Second) -> u8 {
        match s {
            Second::X => {
                match f {
                    First::R => 4,
                    First::P => 1,
                    First::S => 7,
                }
            }
            Second::Y => {
                match f {
                    First::R => {8}
                    First::P => {5}
                    First::S => {2}
                }
            }
            Second::Z => {
                match f {
                    First::R => {3}
                    First::P => {9}
                    First::S => {6}
                }
            }
        }
    }

    fn result2(f: First, s: Second) -> u8 {
        match s {
            Second::X => {0 + f.lose()}
            Second::Y => {3 + f.tie()}
            Second::Z => {6 + f.win()}
        }
    }
}

impl<'a> Day for Day2<'a> {
    const DAY: usize = 2;

    fn create<B: BufRead + 'a>(input: B) -> Self {
        Day2(Box::new(input.lines().map(|ln| {
            let ln = ln.unwrap();
            let mut ws = ln.split_whitespace();
            let f = First::from_str(ws.next().unwrap()).unwrap();
            let s = Second::from_str(ws.next().unwrap()).unwrap();
            (f, s)
        })))
    }

    fn solve_a(self) {
        let r = self.0.fold(0usize, |a, e| {
            a + usize::from(Second::result(e.0, e.1))
        } );
        println!("{}", r);
    }

    fn solve_b(self) {
        println!("{}", self.0.fold(0usize, |a, e|{
            a + usize::from(Second::result2(e.0, e.1))
        }));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test_a(){
        let d =  Day2::create(INPUT.as_bytes());
        d.solve_a();
    }

    #[test]
    fn test_b(){
        let d =  Day2::create(INPUT.as_bytes());
        d.solve_b();
    }
}