#[allow(unused_imports)]
use proconio::{input, marker::Chars};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::f64::consts::PI;
#[allow(unused_imports)]
use std::fmt::Debug;
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
        uv: [(usize, usize); n-1],
    }
    let mut rerooting_dp = RerootingDP::new(
        n,
        RerootingMonoid {
            size: 0,
            dist_sum: 0,
        },
        merge,
        add_root,
    );
    for (u, v) in uv {
        rerooting_dp.add_edge(u - 1, v - 1);
    }
    rerooting_dp.solve();
    for i in 0..n {
        let res = rerooting_dp.dp_all[i].dist_sum;
        println!("{}", res);
    }
}

// TODO: Monoid, merge, liftを実装してRerootingDPに渡す
#[derive(Clone, Debug)]
pub struct RerootingMonoid {
    size: usize,
    dist_sum: usize,
}

fn merge(a: RerootingMonoid, b: RerootingMonoid) -> RerootingMonoid {
    RerootingMonoid {
        size: a.size + b.size,
        dist_sum: a.dist_sum + b.dist_sum,
    }
}

fn add_root(a: RerootingMonoid) -> RerootingMonoid {
    RerootingMonoid {
        size: a.size + 1,
        dist_sum: a.dist_sum + a.size + 1,
    }
}

/// RerootingDP（全方位木DP）
pub struct RerootingDP<T: Clone + Debug> {
    pub size: usize,
    pub graph: Vec<Vec<usize>>,
    pub identity: T,          // 単位元
    pub merge: fn(T, T) -> T, // モノイド同士の二項演算
    pub add_root: fn(T) -> T, // 部分木に根を追加した時に値がどう変わるか
    /// dfs1の結果。根を0とした時の、各頂点vの子孫たちの情報だけをまとめた値 (v自身は含まない)。
    /// `add_root(dp_sub[v])`とすることで、v自身を含む部分木全体の情報となる。
    pub dp_sub: Vec<T>,
    /// dfs2の結果。各頂点vを根としたときの木全体の集約値。これが最終的な答え。
    pub dp_all: Vec<T>,
}

impl<T: Clone + Debug> RerootingDP<T> {
    pub fn new(size: usize, identity: T, merge: fn(T, T) -> T, add_root: fn(T) -> T) -> Self {
        Self {
            size,
            graph: vec![vec![]; size],
            identity: identity.clone(),
            merge,
            add_root,
            dp_sub: vec![identity.clone(); size],
            dp_all: vec![identity.clone(); size],
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.graph[u].push(v);
        self.graph[v].push(u);
    }

    fn dfs1(&mut self, v: usize, parent: Option<usize>) {
        let neighbors = self.graph[v].clone(); // ← 借用を回避するため clone
        let mut acc = self.identity.clone();
        for u in neighbors {
            if Some(u) == parent {
                continue;
            }
            self.dfs1(u, Some(v));
            acc = (self.merge)(acc, (self.add_root)(self.dp_sub[u].clone()));
        }
        self.dp_sub[v] = acc;
    }

    fn dfs2(&mut self, v: usize, parent: Option<usize>, acc_from_parent: T) {
        let n = self.graph[v].len();
        let mut dp_l = vec![self.identity.clone(); n + 1];
        let mut dp_r = vec![self.identity.clone(); n + 1];

        let mut neighbor_monoids = vec![self.identity.clone(); n];
        for i in 0..n {
            let u = self.graph[v][i];
            neighbor_monoids[i] = if Some(u) == parent {
                acc_from_parent.clone()
            } else {
                (self.add_root)(self.dp_sub[u].clone())
            };
        }

        // 前後から累積merge
        for i in 0..n {
            dp_l[i + 1] = (self.merge)(dp_l[i].clone(), neighbor_monoids[i].clone());
        }

        for i in (0..n).rev() {
            dp_r[i] = (self.merge)(dp_r[i + 1].clone(), neighbor_monoids[i].clone());
        }

        self.dp_all[v] = dp_l[n].clone();

        for i in 0..n {
            let u = self.graph[v][i];
            if Some(u) == parent {
                continue;
            }
            let without_u = (self.merge)(dp_l[i].clone(), dp_r[i + 1].clone());
            self.dfs2(u, Some(v), (self.add_root)(without_u));
        }
    }

    pub fn solve(&mut self) -> Vec<T> {
        self.dfs1(0, None);
        println!("dp_sub: {:?}", self.dp_sub);
        self.dfs2(0, None, self.identity.clone());
        println!("dp_all: {:?}", self.dp_all);
        self.dp_all.clone()
    }
}
