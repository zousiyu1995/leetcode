#[test]
fn test() {
    assert_eq!(pick_gifts(vec![25, 64, 9, 4, 100], 4), 29);
    assert_eq!(pick_gifts(vec![1, 1, 1, 1], 4), 4);
}

// 大顶堆（或称优先队列）
pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
    use std::collections::BinaryHeap;
    let mut heap: BinaryHeap<i32> = BinaryHeap::from(gifts);

    for _ in 0..k {
        let max: i32 = heap.pop().unwrap();
        heap.push((max as f64).sqrt() as i32);
    }

    heap.iter().map(|&x| x as i64).sum()
}
