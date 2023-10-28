#[test]
fn test() {
    assert_eq!(min_stone_sum(vec![5, 4, 9], 2), 12);
    assert_eq!(min_stone_sum(vec![4, 3, 6, 7], 3), 12);
}

pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
    use std::collections::BinaryHeap;

    let mut heap: BinaryHeap<i32> = BinaryHeap::from(piles);

    // 每次从拥有最多石子的堆中移除
    for _ in 0..k {
        let pick: i32 = heap.pop().unwrap();
        heap.push(pick - pick / 2);
    }

    heap.iter().sum()
}
