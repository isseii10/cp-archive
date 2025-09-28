use cargo_snippet::snippet;

#[snippet("next_permutation")]
fn next_permutation<T: Ord>(a: &mut [T]) -> bool {
    let n = a.len();
    if n < 2 {
        return false;
    }

    let mut i = n - 2;
    while i != usize::MAX && a[i] >= a[i + 1] {
        if i == 0 {
            break;
        }
        i -= 1;
    }

    if i == 0 && a[0] >= a[1] {
        return false;
    }

    let mut j = n - 1;
    while a[j] <= a[i] {
        j -= 1;
    }

    a.swap(i, j);
    a[i + 1..].reverse();
    true
}
