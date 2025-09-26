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
        m: usize,
        xy: [(usize, usize); m],
    }
    let mut g = vec![vec![]; n];
    let mut indeg = vec![0; n];
    let mut outdeg = vec![0; n];
    for &(x, y) in xy.iter() {
        g[x - 1].push(y - 1);
        indeg[y - 1] += 1;
        outdeg[x - 1] += 1;
    }

    // dp[i]: 頂点iから始めた時の最長パスに含まれる頂点数(最長パス+1)
    // (頂点数にしたのは0を未定の値として使うため)
    let mut dp = vec![0; n];

    // 出次数が0の時、終点なので1
    for (i, &d) in outdeg.iter().enumerate() {
        if d == 0 {
            dp[i] = 1;
        }
    }
    // 有向閉路を持たないので入次数0の頂点が必ず存在し、そこから始めるのが最良
    for (i, &d) in indeg.iter().enumerate() {
        if d == 0 {
            dfs(&g, &mut dp, i);
        }
    }
    println!("{}", dp.iter().max().unwrap() - 1);
}

fn dfs(g: &[Vec<usize>], dp: &mut [usize], cur: usize) {
    if dp[cur] != 0 {
        return;
    }
    for &nxt in g[cur].iter() {
        dfs(g, dp, nxt);
        dp[cur] = max(dp[cur], dp[nxt] + 1);
    }
}
