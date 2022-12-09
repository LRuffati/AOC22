use std::collections::HashMap;
use std::io::BufRead;
use regex::Regex;

use crate::days::Day;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
enum Path {
    Root,
    Any(Vec<String>),
}

impl Path {
    fn root() -> Self {
        Path::Root
    }

    fn add(&self, sub: &str) -> Self{
        match self {
            Path::Root => Path::Any(vec![sub.to_string()]),
            Path::Any(v) => {
                let mut vn: Vec<String> = v.iter().map(|s| s.clone()).collect();
                vn.push(sub.to_string());
                Path::Any(vn)
            }
        }

    }

    fn parent(&self) -> Self {
        match self {
            Path::Root => Path::Root,
            Path::Any(v) => {
                if v.len() == 1 {
                    Path::Root
                } else {
                    Path::Any(v.iter().take(v.len()-1).map(|x| x.clone()).collect())
                }
            }
        }
    }
}

enum Node{
    Dir{p: Path, sz: usize, files: Vec<(String, usize)>, dir: Vec<Path>},
    File(Path, usize)
}
impl Node {
    fn size(&self) -> usize {
        match self {
            Node::Dir { p: _, sz, files: _, dir: _ } => *sz,
            Node::File(_, sz) => *sz
        }
    }

    fn dirsize(&self) -> usize {
        match self {
            Node::File(_, _) => 0,
            _ => self.size()
        }

    }
}

pub struct Day7 {
    fs: HashMap<Path, Node>,
}

impl Day for Day7 {
    const DAY: usize = 7;

    fn create<B: BufRead>(input: B) -> Self {
        let re = Regex::new(r"(?P<cmd>\$ (cd (?P<dest>.+)|ls)|dir (?P<chd_d>.+)|(?P<fls>\d+) (?P<fln>.+))").unwrap();
        let mut fs: HashMap<Path, Node> = HashMap::new();
        let mut curr = Path::root();
        let mut curr_sz = 0;
        let mut curr_fs = Vec::new();
        let mut curr_dr = Vec::new();
        let mut visited_dirs = vec![Path::root()];
        let mut modified = false;

        for l in input.lines().map(|x| x.unwrap()){
            let caps = re.captures(l.as_str()).unwrap();
            if let Some(dest) = caps.name("dest") {
                if modified {
                    //println!("Inserting {:?}, sz: {}", curr, curr_sz);
                    fs.insert(curr.clone(), Node::Dir { p: curr.clone(), sz: curr_sz, files: curr_fs, dir: curr_dr });
                }
                curr_dr = Vec::new();
                curr_fs = Vec::new();
                curr_sz = 0;
                modified = false;

                match dest.as_str() {
                    "/" => {curr = Path::root();}
                    ".." => {curr = curr.parent();}
                    a => {curr = curr.add(a); visited_dirs.push(curr.clone());}
                }
            } else if let Some(dr) = caps.name("chd_d") {
                modified = true;
                curr_dr.push(curr.add(dr.as_str()));
                //println!("{:?} /dir", curr);
                                
            } else if let Some(sz) = caps.name("fls") {
                modified = true;
                let sz = sz.as_str().parse().unwrap();
                curr_sz += sz;
                curr_fs.push((caps.name("fln").unwrap().as_str().to_string(), sz));
                //println!("{:?} /file {} tot {}", curr, sz, curr_sz);
            }
        }
        //println!("Inserting {:?}, sz: {}", curr, curr_sz);
        fs.insert(curr.clone(), Node::Dir { p: curr.clone(), sz: curr_sz, files: curr_fs, dir: curr_dr });

        //println!("Getting totals");

        for d in visited_dirs.into_iter().rev() {
            let mut entry = fs.remove(&d).unwrap();
            match &mut entry {
                Node::Dir { p: _, sz, files: _, dir } => {
                    //println!("Tot sz of {:?}, start: {}", p, *sz);
                    for dc in dir {
                        *sz += fs.get(&dc).unwrap().size();
                    }
                }
                _ => {}
            }
            //println!("{:?}, {}", d, entry.size());
            fs.insert(d.clone(), entry);
        }
        
        return Day7{fs};
    }

    fn solve_a(mut self) {
        let mut tot = 0;
        for d in self.fs.iter_mut() {
            let sz = d.1.dirsize();
            //println!("{:?}, {}", d.0, sz);
            if sz <= 100000 {
                tot += sz;
            }
        }
        println!("Total : {}", tot);
    }

    fn solve_b(mut self) {
        let tot = self.fs.get(&Path::root()).unwrap().size();
        let targ = 30000000 - (70000000 - tot);
        let mut min: usize = tot;
        println!("targ {}", targ);
        for d in self.fs.iter_mut() {
            let sz = d.1.dirsize();
            if (sz >= targ) && (sz < min) {min = sz}
        }
        println!("{}", min);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    #[test]
    fn test_a(){
        let d =  Day7::create(INPUT.as_bytes());
        d.solve_a();
    }

    #[test]
    fn test_b(){
        let d =  Day7::create(INPUT.as_bytes());
        d.solve_b();
    }
}