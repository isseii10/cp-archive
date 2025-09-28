// TODO: Monoid, merge, add_rootを実装してRerootingDPに渡す
use std::fmt::Debug;
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
    pub add_root: fn(T) -> T,
    // dp_sub: dfs1の結果。根を0とした時の、各頂点vの子孫たちの情報だけをまとめた値 (v自身は含まない)。
    // `add_root(dp_sub[v])`とすることで、v自身を含む部分木全体の情報となる。
    pub dp_sub: Vec<T>,
    // dp_all: dfs2の結果。各頂点vを根としたときの木全体の集約値。これが最終的な答え。
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
        let neighbors = self.graph[v].clone();
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
        // println!("dp_sub: {:?}", self.dp_sub);
        self.dfs2(0, None, self.identity.clone());
        // println!("dp_all: {:?}", self.dp_all);
        self.dp_all.clone()
    }
}
