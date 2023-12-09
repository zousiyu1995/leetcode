#[test]
fn test() {
    use method1::nth_ugly_number;

    assert_eq!(nth_ugly_number(10), 12);
    assert_eq!(nth_ugly_number(1), 1);
}

// 优先队列，小顶堆
// 每次用最小的元素乘2、3、5。但要注意去重
// 堆用i64，不然会溢出，leetcode坑人的传统手艺
mod method1 {
    pub fn nth_ugly_number(n: i32) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut heap: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
        let mut ans: i64 = 1;
        heap.push(Reverse(ans));

        for _ in 0..n {
            heap.push(Reverse(ans * 2));
            heap.push(Reverse(ans * 3));
            heap.push(Reverse(ans * 5));
            ans = heap.pop().unwrap().0;
            // 去重
            while !heap.is_empty() && ans == heap.peek().unwrap().0 {
                heap.pop();
            }
        }

        ans as i32
    }
}
