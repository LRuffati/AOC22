use std::fs::File;
use std::io::BufReader;

mod days;

use days::*;

fn main() {
    let pth: String = Curr::path();
    let file = File::open(pth.as_str()).expect("Can't find file");
    let reader = BufReader::new(file);
    let day = Curr::create(reader);
    day.solve_a();

    let file = File::open(pth.as_str()).expect("Can't find file");
    let reader = BufReader::new(file);
    let day = Curr::create(reader);
    day.solve_b();
}
