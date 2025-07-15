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
#[allow(unused_imports)]
use std::isize::MIN;
#[allow(unused_imports)]
use std::usize::MAX;

#[allow(dead_code)]
type Map<K, V> = HashMap<K, V>;
#[allow(dead_code)]
type Set<T> = HashSet<T>;
#[allow(dead_code)]
type Deque<T> = VecDeque<T>;
#[allow(dead_code)]
type Heap<T> = BinaryHeap<T>;

fn main() {
    input! {
        n: usize,
        mut s: [Chars; n],
        t: [Chars; n],
    }

    for _ in 0..4 {
        if check(&s, &t) {
            println!("Yes");
            return;
        }
        s = rotate90(&s);
    }
    println!("No");
}

fn rotate90(s: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n = s.len();
    let mut ret = vec![vec!['.'; n]; n];
    for h in 0..n {
        for w in 0..n {
            let nh = w;
            let nw = n - 1 - h;
            ret[nh][nw] = s[h][w];
        }
    }
    ret
}

fn check(s: &Vec<Vec<char>>, t: &Vec<Vec<char>>) -> bool {
    let n = s.len();

    let ss = trim(n, &s);
    let tt = trim(n, &t);

    if ss.len() != tt.len() {
        return false;
    }
    if ss[0].len() != tt[0].len() {
        return false;
    }

    // println!("ss: {:?}", ss);
    // for r in ss.iter() {
    //     println!("{:?}", r)
    // }
    // println!("tt: {:?}", tt);
    // for r in tt.iter() {
    //     println!("{:?}", r)
    // }

    for i in 0..ss.len() {
        for j in 0..ss[0].len() {
            if ss[i][j] != tt[i][j] {
                return false;
            }
        }
    }
    true
}

fn trim(n: usize, s: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut left = n;
    let mut up = n;
    let mut right = 0;
    let mut down = 0;
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == '#' {
                left = left.min(j);
                up = up.min(i);
                right = right.max(j);
                down = down.max(i);
            }
        }
    }
    // println!(
    //     "left: {}, right: {}, up: {}, down: {}",
    //     left, right, up, down
    // );

    let mut ret = vec![vec!['.'; right - left + 1]; down - up + 1];
    for i in 0..=down - up {
        for j in 0..=right - left {
            ret[i][j] = s[i + up][j + left]
        }
    }
    ret
}
