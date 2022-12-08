use std::{io::BufRead, collections::{HashSet, BinaryHeap, HashMap}, iter::zip, cmp::Reverse};

use super::Day;

#[derive(Clone, Copy, Debug, Hash)]
struct Tree(u8, (usize, usize));

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for Tree {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Eq for Tree {
    
}

impl Ord for Tree {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/// highest is the highest already visited tree in this axis, heap contains
/// the trees which are visible from yet to be visited spots
fn add_tree(t: Tree, highest: &mut Tree, heap: &mut BinaryHeap<Reverse<Tree>>) -> bool{
    if *highest < t { // If tree is taller than the highest it is visible from
                      // the beginning and obscures the following ones which are smaller
        println!("{:?} > {:?}, updating highest", t, highest);
        *highest = t; 
        heap.clear(); // Trees in the heap are smaller or equal than the tallest, so
                      // they end up between two larger trees, won't be visible
        true
    } else {
        // Add the tree to the heap
        while let Some(Reverse(el)) = heap.peek(){
            if (*el).0 <= t.0 { // Trees in the heap are already hidden on one side, this tree would
                                // hide them on the other as well
                println!("{:?} <= {:?}, popping", el, t);
                heap.pop();
            } else {
                break;
            }
        }
        heap.push(Reverse(t));
        false
    }
}

pub struct Day8(HashSet<Tree>);

impl Day for Day8 {
    const DAY: usize = 8;

    fn create(input: std::io::BufReader<std::fs::File>) -> Self {
        let mut vis: HashSet<Tree> = HashSet::new();
        //let all: HashMap<(usize, usize), u8> = HashMap::new();
        let mut x = input.lines().enumerate();

        let mut max_col_up: Vec<Tree> = x.next().unwrap().1.unwrap().as_bytes().iter().enumerate().map(|(i,e)| Tree(*e - 48, (0, i))).collect();
        for i in max_col_up.iter() {
            vis.insert(*i);
        }

        let mut heaps_col: Vec<BinaryHeap<Reverse<Tree>>> = max_col_up.iter().map(|_| BinaryHeap::new()).collect();
        let mut heap_row: BinaryHeap<Reverse<Tree>> = BinaryHeap::new();

        for (r, x) in x{
            let mut l_it = x.unwrap().into_bytes().into_iter().enumerate().map(|(c, e)| {Tree(e-48, (r, c))});
            let mut highest_left = l_it.next().unwrap();
            vis.insert(highest_left);
            
            for (t, (mc, hc)) in zip(l_it, zip(max_col_up.iter_mut().skip(1), heaps_col.iter_mut().skip(1))) {
                let in_hor = add_tree(t, &mut highest_left, &mut heap_row);
                let in_ver = add_tree(t, mc, hc);
                
                if in_hor || in_ver {
                    println!("{:?} visible", t);
                    vis.insert(t);
                }
            }
            for t in heap_row.drain(){
                println!("{:?} visible from right", t.0);
                vis.insert(t.0);
            }
        }
        for mut h in heaps_col.into_iter().skip(1) {
            for t in h.drain(){
                println!("{:?} visible from low", t.0);
                vis.insert(t.0);
            }
        }

        Day8(vis)

    }

    fn solve_a(mut self) {
        println!("{}", self.0.len());
        /*for t in self.0.drain(){
            println!("{:?}", t);
        }*/
    }

    fn solve_b(self) {
        println!()
    }
}
