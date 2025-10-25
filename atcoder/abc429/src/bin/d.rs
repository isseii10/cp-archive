#[allow(unused_imports)]
use amplify::confinement::Collection;
#[allow(unused_imports)]
use proconio::{input, marker::Chars};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::f64::consts::PI;
#[allow(unused_imports)]
use std::io::{self, Write};

#[allow(dead_code)]
type Map<K, V> = HashMap<K, V>;
#[allow(dead_code)]
type Set<T> = HashSet<T>;
#[allow(dead_code)]
type Deque<T> = VecDeque<T>;
#[allow(dead_code)]
type Heap<T> = BinaryHeap<T>;
#[allow(dead_code)]
type Mint = ac_library::ModInt998244353;
// type Mint = ac_library::ModInt1000000007;

fn main() {
    input! {
        n: usize,
        m: usize,
        c: usize,
        a: [usize; n],
    }

    // keyの地点に何人いるか
    let mut cnt = BTreeMap::new();
    for &e in a.iter() {
        if let Some(c) = cnt.get_mut(&e) {
            *c += 1
        } else {
            cnt.push((e, 1))
        }
    }
    let mut sa = vec![];

    for _ in 0..2 {
        for (&place, &num) in cnt.iter() {
            if sa.is_empty() {
                sa.push((place, num));
                continue;
            }
            let (prev_place, prev_num) = sa[sa.len() - 1];
            let p = if place <= prev_place {
                place + m
            } else {
                place
            };
            sa.push((p, prev_num + num))
        }
    }

    let nn = cnt.len() + 1;
    let mut ans = 0;
    let mut r = 1;
    // println!("{:?}", sa);
    for l in 1..nn {
        while r < sa.len() && sa[r].1 - sa[l - 1].1 < c {
            r += 1;
        }
        if r == sa.len() {
            break;
        }
        // println!(
        //     "place: {}, sa[r]: {}, sa[l-1]: {}",
        //     sa[l].0,
        //     sa[r].1,
        //     sa[l - 1].1
        // );
        ans += (sa[l].0 - sa[l - 1].0) * (sa[r].1 - sa[l - 1].1)
    }
    println!("{}", ans)
}
