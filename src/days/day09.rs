use std::{io::BufRead, collections::HashSet};


type Coord = (isize, isize);

trait Rope {
    fn follow(&self, head: &Self) -> Self;
    fn move_r(&mut self, mv: Self) -> ();
    fn follow_move(&mut self, head: &Self) -> ();
}

impl Rope for Coord {
    fn follow(&self, head: &Self) -> Self {
        match (head.0 - self.0, head.1 -self.1) {
            (0, x) if x.abs() > 1 => (0, x.signum()),
            (y, 0) if y.abs() > 1 => (y.signum(), 0),
            (y, x) if (x.abs() >= 2) => (y.signum(), x.signum()),
            (y, x) if (y.abs() >= 2) => (y.signum(), x.signum()),
            _ => (0,0)
        }
    }

    fn move_r(&mut self, mv: Self) -> () {
        self.0 += mv.0;
        self.1 += mv.1;
    }

    fn follow_move(&mut self, head: &Self) -> () {
        let m = self.follow(head);
        self.move_r(m);
    }
}

pub struct Day(Vec<(Coord, Coord)>);

impl super::Day for Day {
    const DAY: usize = 9;

    fn create(input: std::io::BufReader<std::fs::File>) -> Self {
        let (v, _, _) = input.lines().fold((vec![((0,0),(0,0))], (0,0), (0,0)), |(mut v, mut h, mut t), e|{
            let st = e.unwrap();
            let (d, s) = st.split_once(' ').unwrap();
            let s = s.parse::<usize>().unwrap();
            let d: Coord = match d {
                "R" => (0, 1),
                "U" => (1, 0),
                "D" => (-1, 0),
                "L" => (0, -1),
                _ => panic!()
            };
            for _ in 0..s {
                h.move_r(d);
                t.follow_move(&h);
                v.push((h, t))
            }
            (v, h, t)
        });

        return Day(v);
    }

    fn solve_a(mut self) {
        let mut s = HashSet::new();
        for (_, i) in self.0 {
            //println!("{:?}", i);
            s.insert(i);
        }
        println!("{}", s.len());
    }

    fn solve_b(mut self) {
        let mut s = HashSet::new();
        let mut t: [Coord; 8] = [(0,0); 8];
        s.insert(t[7]);
        for (rh, ft) in self.0 {
            let mut h = ft;
            for i in 0..8 {
                let m = t[i].follow(&h);
                if m == (0,0) {break;}
                t[i].move_r(m);
                h = t[i];
            }
            s.insert(t[7]);
        } 
        println!("{}", s.len())
    }
}