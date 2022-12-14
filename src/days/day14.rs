use std::{io::BufRead, collections::HashSet, cmp::{min, max}, hash::Hash};

use nom::{IResult, sequence::{separated_pair, tuple, preceded}, character::complete, bytes::complete::tag, multi::many0};

#[derive(Debug)]
pub struct Day{
    full: HashSet<(i64, i64)>, 
    bott: Vec<(i64, i64)>,
    busy_col: HashSet<i64>,
}

fn parse_seq(inp: &str) -> ((i64,i64), Vec<(i64, i64)>) {
    let mut pair1 = separated_pair(complete::i64::<&str, ()>, tag(","), complete::i64);
    let mut pair2 = separated_pair(complete::i64::<&str, ()>, tag(","), complete::i64);
    let mut list = tuple((pair1, many0(preceded(tag(" -> "), pair2))));
    let (r, (f, mut tl)) = list(inp).unwrap();
    (f, tl)
}

fn nxt(p: (i64, i64)) -> [(i64, i64); 3] {
    [(p.0, p.1+1), (p.0-1, p.1+1), (p.0+1, p.1+1)]
}

fn nxt_free(p: (i64, i64), st: &HashSet<(i64, i64)>) -> Option<(i64, i64)> {
    for i in nxt(p) {
        if !st.contains(&i) {
            return Some(i);
        }
    }
    None
}

fn repr(set: &HashSet<(i64, i64)>, orig: &HashSet<(i64,i64)>, path: &HashSet<(i64, i64)>) {
    let mut vx = Vec::new();
    let mut vy = Vec::new();
    for i in set {
        vx.push(i.0);
        vy.push(i.1);
    }

    vx.sort(); vy.sort();
    let min_x = *vx.get(0).unwrap();
    let min_y = *vy.get(0).unwrap();
    let max_x = vx.pop().unwrap();
    let max_y = vy.pop().unwrap();

    print!("{}[2J", 27 as char);
    for y in min_y..=max_y {
        print!("{y:03}: ");
        for x in min_x..=max_x{
            if set.contains(&(x, y)){
                if orig.contains(&(x,y)){
                    print!("#");
                } else {
                    print!("o");
                }
            } else if path.contains(&(x, y)) {
                print!("x");
            } else {
                print!(".");
            }
        }
        println!()
    }
}

impl super::Day for Day {
    const DAY: usize = 14;

    fn create<B: BufRead>(input: B) -> Self {
        let mut busy = HashSet::new();
        let mut busy_col = HashSet::new();
        let mut bottom: Vec<(i64, i64)> = Vec::new();

        for i in input.lines() {
            let strin = i.unwrap();
            let s = strin.as_str();
            let (mut frm, vec) = parse_seq(s);
            //vec.reverse();

            busy.insert(frm);
            busy_col.insert(frm.0);
            let col_pos = bottom.binary_search_by_key(&(frm.0), |z| (*z).0);
            match col_pos {
                Ok(pos) => {
                    let prev = bottom.get_mut(pos).unwrap();
                    *prev = (prev.0, max(prev.1, frm.1));
                },
                Err(p) => {bottom.insert(p, frm)},
            }

            for i in vec {
                match (i.0 - frm.0, i.1 - frm.1) {
                    (x, 0) => {
                        let dlt = x.signum();
                        for _ in 0..x.abs() {
                            frm.0 += dlt;
                            busy.insert(frm);
                            busy_col.insert(frm.0);
                            let col_pos = bottom.binary_search_by_key(&(frm.0), |z| (*z).0);
                            match col_pos {
                                Ok(pos) => {
                                    let prev = bottom.get_mut(pos).unwrap();
                                    *prev = (prev.0, max(prev.1, frm.1));
                                },
                                Err(p) => {bottom.insert(p, frm)},
                            }
                        }
                    }
                    (0, y) => {
                        let dlt = y.signum();
                        let p = bottom.binary_search_by_key(&(frm.0), |z| (*z).0).expect("Column should have an element already");
                        for _ in 0..y.abs() {
                            frm.1 += dlt;
                            busy.insert(frm);
                        }
                        if dlt == 1 {
                            let prev = bottom.get_mut(p).unwrap();
                            (*prev).1 = max(prev.1, frm.1);
                        }
                    }
                    _ => {panic!()}
                }
            }

        }

        return Day{full: busy, bott: bottom,  busy_col};
    }

    fn solve_a(mut self) {
        let orig = self.full.clone();
        let mut path_s = HashSet::new();

        
        let mut path = Vec::new();
        let mut pos = (500, 0);
        'out: loop {
            let n = nxt(pos);
            for i in n {
                if !self.full.contains(&i) {
                    path.push(pos);
                    path_s.insert(pos);
                    pos = i;
                    continue 'out;
                }
            }
            self.full.insert(pos);
            break;
        }
        let mut cntr = 1;
        
        'out: loop {
            //repr(&self.full, &orig, &path_s);
            pos = path.pop().expect("500,0 should never get filled");
            _ = path_s.remove(&pos);
            let mut nxtf = nxt_free(pos, &self.full);
            while nxtf.is_none() {
                pos = path.pop().unwrap();
                _ = path_s.remove(&pos);
                nxtf = nxt_free(pos, &self.full);
            }
            while let Some(p) = nxtf {
                if !self.busy_col.contains(&p.0) {
                    break 'out;
                }
                let x = self.bott.binary_search_by_key(&p.0, |z| (*z).0).unwrap();
                let bot_col = self.bott.get(x).unwrap().1;
                if p.1 > bot_col {
                    break 'out;
                }
                path.push(pos);
                path_s.insert(pos);
                pos = p;
                nxtf = nxt_free(pos, &self.full);
                if nxtf.is_none(){
                    //println!("{cntr}: {p:?}");
                }
            }
            self.full.insert(pos);
            cntr += 1;
        }
        repr(&self.full, &orig, &path_s);

        println!("{cntr}");
    }

    fn solve_b(mut self) {
        let mut bot = self.bott.iter().map(|(a, b)| *b).reduce(|a, e| max(a, e)).unwrap();
        bot+=2;
        let max_x = self.bott.iter().map(|(a, b)| *a).reduce(|a, e| max(a, e)).unwrap();
        let min_x = self.bott.iter().map(|(a, b)| *a).reduce(|a, e| min(a, e)).unwrap();

        for x in (500-bot-5)..=(500+bot+5) {
            self.full.insert((x, bot));
        }

        let orig = self.full.clone();
        let mut path_s = HashSet::new();

        
        let mut path = Vec::new();
        let mut pos = (500, 0);
        'out: loop {
            let n = nxt(pos);
            for i in n {
                if !self.full.contains(&i) {
                    path.push(pos);
                    path_s.insert(pos);
                    pos = i;
                    continue 'out;
                }
            }
            self.full.insert(pos);
            break;
        }
        let mut cntr = 1;
        
        'out: loop {
            //repr(&self.full, &orig, &path_s);
            //println!("{cntr}");
            let pos_o = path.pop();
            match pos_o {
                Some(p) => {pos = p;},
                None => {cntr += 1; break;},
            }
            _ = path_s.remove(&pos);
            let mut nxtf = nxt_free(pos, &self.full);
            while nxtf.is_none() && !path.is_empty(){
                pos = path.pop().unwrap();
                _ = path_s.remove(&pos);
                nxtf = nxt_free(pos, &self.full);
            }
            while let Some(p) = nxtf {
                path.push(pos);
                path_s.insert(pos);
                pos = p;
                nxtf = nxt_free(pos, &self.full);
                if nxtf.is_none(){
                    //println!("{cntr}: {p:?}");
                }
            }
            self.full.insert(pos);
            cntr += 1;
        }
        repr(&self.full, &orig, &path_s);
        cntr -= 1;
        println!("{cntr}");

    }
}

#[cfg(test)]
mod tests {
    use crate::days::Day;
    static INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
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
