fn topological_sort(
    n: usize,
    graph: &Vec<Vec<usize>>,
    indeg: &mut Vec<usize>,
) -> Option<Vec<usize>> {
    let mut heapq = Heap::new();
    for (v, &d) in indeg.iter().enumerate() {
        if d == 0 {
            heapq.push(Reverse(v));
        }
    }
    // println!("graph: {:?}", graph);
    // println!("indeg: {:?}", indeg);

    let mut order = vec![];

    while let Some(Reverse(p)) = heapq.pop() {
        order.push(p + 1);
        for &c in graph[p].iter() {
            indeg[c] -= 1;
            if indeg[c] == 0 {
                heapq.push(Reverse(c));
            }
        }
    }
    if order.len() == n {
        Some(order)
    } else {
        // サイクルがあり、トポロジカルソートできなかった
        None
    }
}
