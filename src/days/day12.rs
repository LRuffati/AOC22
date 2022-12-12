use std::{io::BufRead, cmp::{max, min, Reverse}, collections::BinaryHeap};

#[derive(Debug)]
pub struct Day{
    points: Vec<Point>, 
    start: Coord, 
    end: Coord, 
    n_rows: usize, 
    n_cols: usize
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Coord((usize, usize));

impl Coord {
    fn to_idx(&self, cols: usize) -> usize {
        let v =cols*self.0.0 + self.0.1;
        v
    }

    fn neighs(&self, cols: usize, rows: usize) -> Vec<Coord> {
        let mut v = Vec::new();
        let mut xs = vec![self.0.0];
        if self.0.0 > 0 {
            xs.push(self.0.0 - 1);
        }
        if self.0.0 < rows-1 {
            xs.push(self.0.0 + 1)
        }
        let mut ys = vec![self.0.1];
        if self.0.1 > 0 {
            ys.push(self.0.1 - 1);
        }
        if self.0.1 < cols-1 {
            ys.push(self.0.1 + 1)
        }
        for r in xs {
            for c in &ys {
                let co = Coord((r, *c));
                if co.dist(self) == 1 {
                    v.push(co);
                }
            }
        }
        v
    }

    fn dist(&self, othr: &Self) -> usize {
        let (x1, y1) = self.0;
        let (x2, y2) = othr.0;
        max(x1, x2) + max(y1, y2) - min(x1, x2) - min(y1, y2)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point(u8, Coord);

impl super::Day for Day {
    const DAY: usize = 12;

    fn create<B: BufRead>(input: B) -> Self {
        let mut v = Vec::new();
        let mut st = None;
        let mut ed = None;
        let mut cols = None;
        let mut rows: Option<usize> = None;


        for (r, l) in input.lines().enumerate() {
            let mut last_c = 0;
            for (c, e) in l.unwrap().as_bytes().into_iter().enumerate() {
                last_c = c;
                match e {
                    b'S' => {v.push(Point(0, Coord((r, c)))); st = Some(Coord((r, c)));}
                    b'E' => {v.push(Point(b'z'-b'a', Coord((r, c)))); ed = Some(Coord((r, c)));}
                    h => {v.push(Point(h-b'a', Coord((r, c))));}
                }
            }
            cols = Some(last_c+1);
            rows = Some(r+1);
        }
        return Day{
            points: v,
            start: st.unwrap(),
            end: ed.unwrap(),
            n_rows: rows.unwrap(),
            n_cols: cols.unwrap(),
        };
    }

    fn solve_a(self) {
        let cols = self.n_cols;
        let rows = self.n_rows;
        let mut to_test = BinaryHeap::new();
        let mut vis: Vec<Option<usize>> = self.points.iter().map(|_x| None).collect();

        let score = move |c: Coord, steps: usize| -> Reverse<usize> {Reverse(steps + c.dist(&self.end))};
        to_test.push((score(self.start, 0), *(self.points.get(self.start.to_idx(cols)).unwrap()) ));

        let mut res = None;
        while let Some((Reverse(s), p)) = to_test.pop() {
            let steps = s - p.1.dist(&self.end);
            for i in p.1.neighs(cols, rows) {
                let idx = i.to_idx(cols);
                let n = self.points.get(idx);
                let n = *n.unwrap();
                if n.0 <= p.0+1 {
                    if n.1 == self.end {
                        res = Some(steps + 1);
                        break;
                    }
                    let visb = vis.get_mut(n.1.to_idx(cols)).unwrap();
                    if visb.is_none() || visb.unwrap() > steps+1 {
                        to_test.push((score(n.1, steps+1), n));
                        *visb=Some(steps+1);

                    }
                }
            }
        }

        println!("{}", res.unwrap());
    }

    fn solve_b(self) {
        let cols = self.n_cols;
        let rows = self.n_rows;
        let mut to_test = BinaryHeap::new();
        let mut vis: Vec<Option<usize>> = self.points.iter().map(|_x| None).collect();

        let score = move |p: Point, steps: usize| -> Reverse<usize> {Reverse(steps + p.0 as usize)};

        let endp = *self.points.get(self.end.to_idx(cols)).unwrap();
        to_test.push((score(endp, 0), endp ));

        let mut res = None;
        while let Some((Reverse(s), p)) = to_test.pop() {
            let steps = s - p.0 as usize;
            for i in p.1.neighs(cols, rows) {
                let idx = i.to_idx(cols);
                let n = self.points.get(idx);
                let n = *n.unwrap();
                if n.0 > p.0 || (n.0 <= p.0 && (n.0+1) >= p.0) {
                    if n.0 == 0 {
                        if res.is_none() || res.unwrap() > steps {
                            res = Some(steps + 1);
                            break;
                        }
                    }
                    let visb = vis.get_mut(n.1.to_idx(cols)).unwrap();
                    if visb.is_none() || visb.unwrap() > steps+1 {
                        to_test.push((score(n, steps+1), n));
                        *visb=Some(steps+1);

                    }
                }
            }
        }

        println!("{}", res.unwrap());
    }
}

#[cfg(test)]
mod tests {
    use crate::days::Day;
    static INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
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
