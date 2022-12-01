use std::fs::File;
use std::io::BufReader;

mod days;
use days::*;

fn main() {
    const PTH: &str = "inputs/01";
    let file = File::open(PTH).expect("Can't find file");
    let reader = BufReader::new(file);
    let day = Curr::create(reader);
    day.solve_a();

    let file = File::open(PTH).expect("Can't find file");
    let reader = BufReader::new(file);
    let day = Curr::create(reader);
    day.solve_b();
}
