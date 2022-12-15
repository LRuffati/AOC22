use std::{io::BufRead, collections::{BinaryHeap, HashSet}, cmp::max};

use itertools::any;
use regex::Regex;

type Coord = (i64, i64);
type Range = (i64, i64);

#[derive(Debug)]
struct Scan {
    /// Distance between sensor and beacon
    dist: u64,
    sensor: Coord,
    beacon: Coord,
}

#[derive(Debug)]
pub struct Day(Vec<Scan>, i64, i64);

fn dist(s: Coord, b: Coord) -> u64 {
    let dx = s.0.abs_diff(b.0);
    let dy = s.1.abs_diff(b.1);
    //println!("{s:?} - {b:?} = {dx}, {dy}");
    dx+dy
}

struct MarginIter {
    center: Coord,
    last: Option<Coord>,
    first: Coord,
    finished: bool,
}
impl Iterator for MarginIter {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {return None;}
        match self.last {
            Some((x, y)) => {
                let rdx = x-self.center.0;
                let rdy = y-self.center.1;
                let dy = match rdx.signum() {
                    0 => {-rdy.signum()},
                    xd => {-xd},
                };
                let dx = match rdy.signum() {
                    0 => -rdx.signum(),
                    yd => yd,
                };
                let n = (x+dx, y+dy);
                
                if n==self.first {
                    self.finished = true;
                    None
                } else {
                    self.last = Some(n);
                    self.last
                }
            },
            None => {
                self.last = Some(self.first);
                Some(self.first)
            },
        }
    }
}

impl MarginIter {
    fn new(center: Coord, range: u64) -> Self {
        Self { center, last: None, first: (center.0, center.1+1+range as i64), finished: false }
    }
}

impl super::Day for Day {
    const DAY: usize = 15;

    fn create<B: BufRead>(input: B) -> Self {
        let mut re = Regex::new(r"Sensor at x=(?P<s_x>-?\d+), y=(?P<s_y>-?\d+): closest beacon is at x=(?P<b_x>-?\d+), y=(?P<b_y>-?\d+)").unwrap();
        let v = input.lines().map(|l|{
            let l = l.unwrap();
            let coll = re.captures(l.as_str()).unwrap();
            let s_x = coll.name("s_x").unwrap().as_str().parse().unwrap();
            let s_y = coll.name("s_y").unwrap().as_str().parse().unwrap();
            let b_x = coll.name("b_x").unwrap().as_str().parse().unwrap();
            let b_y = coll.name("b_y").unwrap().as_str().parse().unwrap();
            let s = (s_x, s_y);
            let b = (b_x, b_y);
            Scan { dist: dist(s, b), sensor: s, beacon: b }
        }).collect();
        
        return Day(v, 2000000, 4000000);
    }

    fn solve_a(mut self) {
        let y_scan: i64 = self.1;
        let mut ranges_cov: Vec<Range> = self.0.iter().filter_map(|Scan { dist, sensor: (x, y), beacon: _ }| {
            let offs = y.abs_diff(y_scan);
            if offs > *dist {
                None
            } else {
                let rem = (dist-offs) as i64;
                Some((*x-rem, *x+rem))
            }
        }).collect();
        ranges_cov.sort_by_key(|z| (*z).0);

        let mut tot = 0;
        let mut itr = ranges_cov.into_iter();
        let (mut beg, mut end) = itr.next().unwrap();
        for (b_c, e_c) in itr {
            if b_c > end {
                tot += beg.abs_diff(end) + 1;
                beg = b_c;
                end = e_c;
            } else {
                end = max(end, e_c);
            }
        }
        tot += beg.abs_diff(end) + 1;
        println!("{tot} before removing beacons");
        let beacs: HashSet<Coord> = self.0.iter().filter(|scan| scan.beacon.1 == y_scan).map(|x|x.beacon.clone()).collect();
        tot -= beacs.len() as u64;
        println!("{tot}");
    }

    fn solve_b(mut self) {
        let mut res = None;
        'out: for i in self.0.iter(){
            for p in MarginIter::new(i.sensor, i.dist) {
                if p.0<0 || p.1<0 || p.0 >self.2 || p.1 > self.2 {
                    continue;
                }
                let mut any_inside = false;
                for s in self.0.iter() {
                    let center = s.sensor;
                    let d_c_p = dist(center, p);
                    let d_ref = s.dist;
                    if d_c_p <= d_ref {
                        any_inside = true;
                        break;
                    }
                }
                /* = self.0.iter().any(|s|{
                    dist(s.beacon, p) <= s.dist
                });*/
                if !any_inside {
                    res = Some(p);
                    println!("{p:?}");
                    break 'out;
                }
            }
        }
        let res = res.expect("No valid found");
        let r = res.0 * 4000000 + res.1;
        println!("{res:?} = {r}")
    }
}

#[cfg(test)]
mod tests {
    use crate::days::Day;

    use super::MarginIter;
    static INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

    #[test]
    fn test_parse() {
        let d = super::Day::create(INPUT.as_bytes());
        for i in d.0 {
            println!("{:?}", i);
        }
    }

    #[test]
    fn test_iter(){
        for i in MarginIter::new((1,0), 2) {
            println!("{i:?}");
        }
    }

    #[test]
    fn test_a(){
        let mut d =  super::Day::create(INPUT.as_bytes());
        d.1 = 10;
        d.solve_a();
    }

    #[test]
    fn test_b(){
        let mut d =  super::Day::create(INPUT.as_bytes());
        d.2 = 20;
        d.solve_b();
    }
}
