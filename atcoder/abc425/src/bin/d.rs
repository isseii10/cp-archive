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
// type Mint = ac_library::ModInt1000000007;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let mut q = Deque::new();
    let mut ss = vec![vec![('.', 0); w]; h];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                q.push_back((i, j, 1));
                ss[i][j] = ('#', 1);
            }
        }
    }
    let d = [(1 as isize, 0 as isize), (0, 1), (-1, 0), (0, -1)];
    while let Some((i, j, level)) = q.pop_front() {
        for (di, dj) in d {
            let ni = i as isize + di;
            let nj = j as isize + dj;
            if !inside(ni, nj, h, w) {
                continue;
            }
            if ss[ni as usize][nj as usize].0 == '#' {
                continue;
            }
            // 黒が1つだけなことを確認
            let mut black = 0;
            for (k, l) in d {
                let nni = ni + k;
                let nnj = nj + l;
                if !inside(nni, nnj, h, w) {
                    continue;
                }
                let (nni, nnj) = (nni as usize, nnj as usize);
                let (c, a) = ss[nni][nnj];
                if c == '#' && a <= level {
                    black += 1;
                }
            }
            if black == 1 {
                ss[ni as usize][nj as usize] = ('#', level + 1);
                q.push_back((ni as usize, nj as usize, level + 1));
            }
        }
    }
    // for i in 0..h {
    //     for j in 0..w {
    //         let (c, _) = ss[i][j];
    //         print!("{}", c);
    //     }
    //     println!()
    // }
    // for i in 0..h {
    //     for j in 0..w {
    //         let (_, l) = ss[i][j];
    //         print!("{}", l);
    //     }
    //     println!()
    // }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            let (c, _) = ss[i][j];
            if c == '#' {
                ans += 1;
            }
        }
    }
    println!("{}", ans)
}

fn inside(i: isize, j: isize, h: usize, w: usize) -> bool {
    0 <= i && i < h as isize && 0 <= j && j < w as isize
}
