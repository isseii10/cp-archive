// -*- coding:utf-8-unix -*-

#[allow(unused_imports)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let aa: Vec<usize> = std::iter::once(0).chain(a).collect();
    let mut b = vec![0; n + 1];

    for i in (1..=n).rev() {
        let mut odd = 0; // 0 or 1
        for j in (1..=n / i).rev() {
            if j == 1 {
                // println!("aa[i]: {}, odd: {}", aa[i], odd);
                b[i] = aa[i] ^ odd;
            } else {
                // println!("b[i*j]: {}, odd: {}", b[i * j], odd);
                odd = b[i * j] ^ odd;
            }
        }
    }

    let ans_n = b.iter().skip(1).filter(|&&x| x == 1).count();
    println!("{}", ans_n);
    if ans_n == 0 {
        return;
    }
    let ans: Vec<usize> = b
        .iter()
        .enumerate()
        .skip(1)
        .filter_map(|(i, &x)| if x == 1 { Some(i) } else { None })
        .collect();
    for (i, x) in ans.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", x);
    }
    println!();
}
