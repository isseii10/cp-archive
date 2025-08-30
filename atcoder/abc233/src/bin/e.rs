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
        mut x: Chars,
    }
    let mut disit_sum = 0;
    for i in 0..x.len() {
        disit_sum += (x[i] as u8 - b'0') as usize;
    }
    let mut disits = vec!['0'; x.len()];
    let mut carried = 0;
    for i in (0..x.len()).rev() {
        let mut d = disit_sum + carried;
        disits[i] = char::from_digit((d % 10) as u32, 10).unwrap();
        d /= 10;
        carried = d;
        disit_sum -= (x[i] as u8 - b'0') as usize;
    }
    let prefix = if carried > 0 {
        carried.to_string()
    } else {
        String::new()
    };

    println!("{}", prefix + disits.iter().collect::<String>().as_str());
}
