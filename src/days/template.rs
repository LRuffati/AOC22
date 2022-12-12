use std::io::BufRead;

#[derive(Debug)]
pub struct Day();

impl super::Day for Day {
    const DAY: usize ;

    fn create<B: BufRead>(input: B) -> Self {
        return Day();
    }

    fn solve_a(mut self) {
        println!("");
    }

    fn solve_b(mut self) {
        println!("")
    }
}

#[cfg(test)]
mod tests {
    use crate::days::Day;
    static INPUT: &str = "";

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
