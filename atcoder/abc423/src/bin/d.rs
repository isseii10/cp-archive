#[allow(unused_imports)]
use amplify::confinement::Collection;
use itertools::Itertools;
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
        k: usize,
        abc: [(usize, usize, usize); n],
    }

    let mut ans = vec![0; n];
    let mut num_in_store = 0;
    let mut now = 0;
    let mut out_queue = Heap::new();

    for i in 0..n {
        let (a, b, c) = abc[i];
        now = max(now, a); // aに到着するのでnowよりaの方が大きい場合はaになるまで待つ

        // 入れられない時は、入れられるようになるまで出す(どうせi+1の団体はiの団体が入ってからじゃないと先頭にならないので、iが入れるようになるまで時を進めても問題ない)
        while num_in_store + c > k {
            if let Some(&(Reverse(ot), oc)) = out_queue.peek() {
                // 最も早く出る団体を出す
                out_queue.pop();
                num_in_store -= oc;
                now = max(now, ot);
            }
        }

        // 入れられるようになったので入れる
        num_in_store += c;
        ans[i] = now;
        out_queue.push((Reverse(now + b), c));
    }

    println!("{}", ans.iter().join("\n"))
}
