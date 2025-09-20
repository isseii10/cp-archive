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
        n: usize,
    }
    let n2 = 2 * n;
    let mut a = vec![vec![0; n2]; n2];
    for i in 0..n2 {
        for j in i + 1..n2 {
            input! {
                v: usize,
            }
            a[i][j] = v;
            a[j][i] = v;
        }
    }
    // println!("{:?}", a);
    let mut ans = 0;
    dfs(&a, n2, &mut ans, 0, 0);
    println!("{}", ans);
}

fn dfs(a: &[Vec<usize>], n2: usize, ans: &mut usize, s: usize, x: usize) {
    let mut u = None;
    let mut ns = s;
    for i in 0..n2 {
        if (s >> i) & 1 == 0 {
            // まだ選んでない中で最小をuとする
            u = Some(i);
            ns |= 1 << i;
            break;
        }
    }
    if u.is_none() {
        *ans = max(*ans, x);
        return;
    }

    let u = u.unwrap();

    for v in 0..n2 {
        if (ns >> v) & 1 == 0 {
            // まだ選んでない
            ns |= 1 << v;
            dfs(a, n2, ans, ns, x ^ a[u][v]);
            ns &= !(1 << v);
        }
    }
}
