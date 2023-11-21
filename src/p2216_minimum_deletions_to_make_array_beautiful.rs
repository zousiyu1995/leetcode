#[test]
fn test() {
    assert_eq!(min_deletion(vec![1, 1, 2, 3, 5]), 1);
    assert_eq!(min_deletion(vec![1, 1, 2, 2, 3, 3]), 2);
}

// https://leetcode.cn/problems/minimum-deletions-to-make-array-beautiful/

pub fn min_deletion(nums: Vec<i32>) -> i32 {
    // 用栈表示美丽数组
    let mut stack: Vec<i32> = vec![];

    for &num in nums.iter() {
        // 栈长度是奇数，检查栈顶元素和待入栈元素
        if stack.len() % 2 == 1 {
            // 两者相等，删除；不相等，入栈
            if *stack.last().unwrap() == num {
                continue;
            } else {
                stack.push(num);
            }
        }
        // 栈长度是偶数，直接入栈
        else {
            stack.push(num);
        }
    }

    // 遍历结束时，如果栈是奇数，移除栈顶元素。栈顶元素并没有意义
    if stack.len() % 2 == 1 {
        stack.pop();
    }

    (nums.len() - stack.len()) as i32
}
