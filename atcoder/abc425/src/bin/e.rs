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
type Mint = ac_library::ModInt;
// type Mint = ac_library::ModInt1000000007;

fn main() {
    input! {
        t: usize,
        m: u32,
    }
    Mint::set_modulus(m);
    let comb = CombPascal::new(5001);

    for _ in 0..t {
        solve(&comb);
    }
}

fn solve(comb: &CombPascal) {
    input! {
        n: usize,
        c: [usize; n],
    }
    let mut ans = Mint::new(1);
    let mut sum = 0;
    for v in c {
        sum += v;
        ans = ans * comb.combination(sum, v);
    }
    println!("{}", ans);
}

/// 二項係数 nCr
/// パスカルの三角形で nCr を求める構造体
/// mが合成数の場合(ConbFactorialが使えない場合)に使用する
pub struct CombPascal {
    pascal: Vec<Vec<Mint>>,
}
impl CombPascal {
    /// n までの nCr を前計算
    pub fn new(n: usize) -> Self {
        let mut pascal = vec![vec![Mint::new(0); n + 1]; n + 1];
        pascal[0][0] = Mint::new(1);
        for i in 0..n {
            for j in 0..=i {
                pascal[i + 1][j] = pascal[i + 1][j] + pascal[i][j];
                pascal[i + 1][j + 1] = pascal[i + 1][j + 1] + pascal[i][j];
            }
        }
        Self { pascal }
    }
    /// nCr
    pub fn combination(&self, n: usize, r: usize) -> Mint {
        if r > n {
            return Mint::new(0);
        }
        self.pascal[n][r]
    }
}
