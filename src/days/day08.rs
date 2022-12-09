use std::{io::BufRead, collections::{HashSet, VecDeque, BTreeMap}, iter::zip, cmp::{Reverse, min}};

use super::Day;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct Tree(u8, (usize, usize));

/// highest is the highest already visited tree in this axis, heap contains
/// the trees which are visible from yet to be visited spots
fn add_tree(t: Tree, highest: &mut Tree, heap: &mut Vec<Tree>) -> bool{
    if highest.0 < t.0 { // If tree is taller than the highest it is visible from
                      // the beginning and obscures the following ones which are smaller
        //println!("{:?} > {:?}, updating highest", t, highest);
        *highest = t; 
        heap.clear(); // Trees in the heap are smaller or equal than the tallest, so
                      // they end up between two larger trees, won't be visible
        true
    } else {
        // Add the tree to the heap
        *heap = heap.iter_mut().filter(|x| (x).0 > t.0).map(|z|*z).collect();
        heap.push(t);
        false
    }
}

pub struct Day8(VecDeque<VecDeque<Tree>>);

impl Day for Day8 {
    const DAY: usize = 8;

    fn create(input: std::io::BufReader<std::fs::File>) -> Self {
        let x: _ = input.lines().into_iter().enumerate().map(|(r, l)|{
            l.unwrap().as_bytes().iter().enumerate().map(|(c, e)| Tree(*e - 48, (r, c))).collect()
        }).collect();
        Day8(x)
    }

    fn solve_a(mut self) {
        let mut vis: HashSet<Tree> = HashSet::new();
        //let all: HashMap<(usize, usize), u8> = HashMap::new();
        let mut iter = self.0.into_iter();
        let mut max_col_up = iter.next().unwrap();
        for i in max_col_up.iter() {
            vis.insert(*i);
        }

        let mut heaps_col: Vec<Vec<Tree>> = max_col_up.iter().map(|_| Vec::new()).collect();
        
        for x in iter{
            let mut heap_row: Vec<Tree> = Vec::new();
            let mut l_it = x.into_iter();
            let mut highest_left = l_it.next().unwrap();
            vis.insert(highest_left);
            
            for (t, (mc, hc)) in zip(l_it, zip(max_col_up.iter_mut().skip(1), heaps_col.iter_mut().skip(1))) {
                let in_hor = add_tree(t, &mut highest_left, &mut heap_row);
                let in_ver = add_tree(t, mc, hc);
                
                if in_hor || in_ver {
                    //println!("{:?} visible", t);
                    vis.insert(t);
                }
            }
            for t in heap_row.iter(){
                //println!("{:?} visible from right", t.0);
                vis.insert(*t);
            }
        }
        for mut h in heaps_col.into_iter().skip(1) {
            for t in h {
                //println!("{:?} visible from low", t.0);
                vis.insert(t);
            }
        }
        println!("{}", vis.len());
        /*for t in self.0.drain(){
            println!("{:?}", t);
        }*/
    }

    fn solve_b(mut self) {
        resize_vec(&mut self.0);
        self.0.iter_mut().for_each(resize_vec);
        // https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search_by
        let mut views: BTreeMap<Tree, usize> = BTreeMap::new();
        
        let mut col_map: BTreeMap<usize, Vec<Tree>> = BTreeMap::new();
        for t in self.0.iter().flatten() {
            let c = (t.1).1;
            let v = col_map.get_mut(&c);
            match v {
                Some(cl) => {cl.push(*t)},
                None => {col_map.insert(c, vec![*t]);},
            };
        }

        for line in self.0.into_iter() {
            find_view::<true>(line.into_iter(), &mut views);
        }

        for col in col_map.into_values(){
            find_view::<false>(col.into_iter(), &mut views);
        }
        for e in views.iter(){
            println!("{:?} -> {}", e.0, e.1);
        }

        let x = *views.values().max().unwrap();        

        println!("{}", x)
    }
}

fn find_view<const t: bool>(line: impl IntoIterator<Item=Tree>, map: &mut BTreeMap<Tree, usize>) {
    let mut add_to_view = |tree: Tree, view: usize| -> () {map.entry(tree).and_modify(|v| {*v = *v * view}).or_insert(view);};
    let lambda = if t {
        |x: &Tree| -> usize {(x.1).1}
    } else {
        |x: &Tree| -> usize {(x.1).0}
    };

    let mut sorted: Vec<_> = line.into_iter().collect();
    let len_v = sorted.len()+2;
    sorted.sort_by_key(|x| (x.0, Reverse(lambda(x))));

    let mut full = Vec::<usize>::new();
    
    while let Some(tree) = sorted.pop() {
        let h = tree.0;
        let coord = lambda(&tree);
        let idx = full.binary_search(&coord).unwrap_err(); // to the right
        let left_dist = if idx==0 {coord} else {coord - *(full.get(idx-1).unwrap())};
        let right_dist_hp1 = full.get(idx).unwrap_or(&(len_v-1)) - coord;
        full.insert(idx, coord);

        match sorted.last() {
            Some(tr) if tr.0 == h => {
                let right_dist_hp2 = lambda(tr) - coord;
                add_to_view(tree, left_dist*min(right_dist_hp1, right_dist_hp2));
                continue;
            },
            _ => {
                add_to_view(tree, left_dist*right_dist_hp1);
            },
        }
    }


    
}

fn resize_vec<T>(v: &mut VecDeque<T>){
    v.pop_back();
    v.pop_front();
}