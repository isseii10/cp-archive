use cargo_snippet::snippet;

/// 次の出現位置
/// next[i][c] := 位置i以降で文字cが最初に出現する位置
/// 出現しない場合はs.len()
#[snippet("next_pos")]
fn next_pos(s: &[char]) -> Vec<Vec<usize>> {
    let n = s.len();
    let mut next = vec![vec![n; 26]; n + 1];
    for i in (0..n).rev() {
        next[i] = next[i + 1].clone();
        next[i][(s[i] as u8 - b'a') as usize] = i;
    }
    next
}
