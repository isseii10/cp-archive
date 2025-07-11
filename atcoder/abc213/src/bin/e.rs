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
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let jump: Vec<(i64, i64)> = (-2..=2)
        .flat_map(|x: i64| {
            (-2..=2).filter_map(move |y: i64| {
                if x.abs() + y.abs() <= 3 && (x != 0 || y != 0) {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .collect();

    let d = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut cost = vec![vec![MAX; w]; h];
    cost[0][0] = 0;

    let mut dq = Deque::new();
    dq.push_back((0, 0));

    while !dq.is_empty() {
        let (x, y) = dq.pop_front().unwrap();
        // cost 0
        for (dx, dy) in &d {
            let nx = x as i64 + dx;
            let ny = y as i64 + dy;
            if !is_insize(nx, ny, h, w) {
                continue;
            }
            if s[nx as usize][ny as usize] == '#' {
                continue;
            }
            if cost[nx as usize][ny as usize] <= cost[x][y] {
                continue;
            }
            cost[nx as usize][ny as usize] = cost[x][y];
            dq.push_front((nx as usize, ny as usize));
        }
        // cost 1
        for (dx, dy) in &jump {
            let nx = x as i64 + dx;
            let ny = y as i64 + dy;
            if !is_insize(nx, ny, h, w) {
                continue;
            }
            if cost[nx as usize][ny as usize] <= cost[x][y] + 1 {
                continue;
            }
            cost[nx as usize][ny as usize] = cost[x][y] + 1;
            dq.push_back((nx as usize, ny as usize));
        }
    }
    println!("{}", cost[h - 1][w - 1]);
}

fn is_insize(x: i64, y: i64, h: usize, w: usize) -> bool {
    x >= 0 && y >= 0 && x < h as i64 && y < w as i64
}
