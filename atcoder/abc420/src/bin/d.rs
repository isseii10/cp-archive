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
#[allow(dead_code)]
type Mint = ac_library::ModInt998244353;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
    }
    let mut dp = vec![vec![vec![MAX; 2]; w]; h];
    let mut sh = 0;
    let mut sw = 0;
    let mut gh = 0;
    let mut gw = 0;
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 'S' {
                sh = i;
                sw = j;
            }
            if a[i][j] == 'G' {
                gh = i;
                gw = j;
            }
        }
    }
    dp[sh][sw][0] = 0;

    let mut q = Deque::new();
    q.push((sh, sw, 0));

    let d = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

    while let Some((ch, cw, cs)) = q.pop_front() {
        for &(dh, dw) in d.iter() {
            let nh = ch as isize + dh;
            let nw = cw as isize + dw;
            let mut ns = cs;
            if nh < 0 || nh >= h as isize || nw < 0 || nw >= w as isize {
                continue;
            }
            let (nh, nw) = (nh as usize, nw as usize);
            if a[nh][nw] == '#' {
                continue;
            }
            if cs == 1 && a[nh][nw] == 'o' {
                continue;
            }
            if cs == 0 && a[nh][nw] == 'x' {
                continue;
            }

            if a[nh][nw] == '?' {
                ns = 1 - cs;
            }

            if dp[nh][nw][ns] > dp[ch][cw][cs] + 1 {
                dp[nh][nw][ns] = dp[ch][cw][cs] + 1;
                q.push((nh, nw, ns));
            }
        }
    }

    if dp[gh][gw][0] == MAX && dp[gh][gw][1] == MAX {
        println!("-1");
    } else {
        println!("{}", min(dp[gh][gw][0], dp[gh][gw][1]));
    }
}
