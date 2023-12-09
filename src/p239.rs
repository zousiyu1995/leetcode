#[test]
fn test() {
    let nums1: Vec<i32> = [1, 3, -1, -3, 5, 3, 6, 7].to_vec();
    assert_eq!(max_sliding_window(nums1, 3), [3, 3, 5, 5, 6, 7].to_vec());

    let nums2: Vec<i32> = [1].to_vec();
    assert_eq!(max_sliding_window(nums2, 1), [1].to_vec());
}

// 单调队列
// 很难想到
pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::VecDeque;

    let k: usize = k as usize;
    let mut deque: VecDeque<usize> = VecDeque::new(); // 储存元素下标
    let mut ans: Vec<i32> = vec![];

    // 用第一个窗口初始化队列
    for i in 0..k {
        // 维护队列的单调性，[front > x > back]
        while !deque.is_empty() && nums[i] >= nums[*deque.back().unwrap()] {
            deque.pop_back();
        }
        deque.push_back(i);
    }
    // 更新答案
    ans.push(nums[*deque.front().unwrap()]);

    // 移动窗口
    for i in k..nums.len() {
        // 维护队列的单调性，[front > x > back]
        while !deque.is_empty() && nums[i] >= nums[*deque.back().unwrap()] {
            deque.pop_back();
        }
        deque.push_back(i);
        // 去掉不在窗口内的元素
        while *deque.front().unwrap() <= i - k {
            deque.pop_front();
        }
        // 更新答案
        ans.push(nums[*deque.front().unwrap()]);
    }

    ans
}
