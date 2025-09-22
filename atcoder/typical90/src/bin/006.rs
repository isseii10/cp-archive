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
        k: usize,
        s: Chars,
    }
    let next = next_pos(&s);
    let mut ans = vec![];
    let mut prev_pos = 0;
    for i in 0..k {
        for c in 0..26 {
            let pos = next[prev_pos][c];
            if n < pos + k - i {
                // この文字を選ぶと、残りの文字数が足りなくなる
                continue;
            }
            ans.push((c as u8 + b'a') as char);
            prev_pos = pos + 1;
            break;
        }
    }
    println!("{}", ans.iter().collect::<String>())
}

// 次の出現位置
// next[i][c] := 位置i以降で文字cが最初に出現する位置
// 出現しない場合はs.len()
fn next_pos(s: &[char]) -> Vec<Vec<usize>> {
    let n = s.len();
    let mut next = vec![vec![n; 26]; n + 1];
    for i in (0..n).rev() {
        next[i] = next[i + 1].clone();
        next[i][(s[i] as u8 - b'a') as usize] = i;
    }
    next
}
