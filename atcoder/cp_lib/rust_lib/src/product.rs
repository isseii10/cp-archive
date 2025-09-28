use cargo_snippet::snippet;

// n個の要素からk個を重複ありで選ぶ順列を生成する関数
// NOTE: 重複ありの順列はitertools 0.11にないので自作
#[snippet]
fn product<T: Clone>(values: &[T], k: usize, current: &mut Vec<T>, results: &mut Vec<Vec<T>>) {
    if current.len() == k {
        results.push(current.clone());
        return;
    }
    for v in values {
        current.push(v.clone());
        product(values, k, current, results);
        current.pop();
    }
}
