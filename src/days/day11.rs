use std::io::BufRead;
pub struct Day();

impl super::Day for Day {
    const DAY: usize = 10;

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