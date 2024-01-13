// https://leetcode.cn/problems/largest-rectangle-in-histogram/description/

// 对于位置i来说，要形成最大面积，需要向左找第一个小于i的位置left，同时向右找第一个小于i的位置right。最大面积为heights[i] * (right - left - 1)
// 所以，问题变为如何寻找left和right
// 最简单的思路是暴力法
mod method1 {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let n: usize = heights.len();
        let mut ans: i32 = 0;

        for i in 0..n {
            let mut left: i32 = i as i32;
            let mut right: i32 = i as i32;
            // 向左找第一个小于i的位置，该位置可能出现在-1
            while left >= 0 && heights[left as usize] >= heights[i] {
                left -= 1;
            }
            // 向右找第一个小于i的位置，该位置可能出现在n
            while right < n as i32 && heights[right as usize] >= heights[i] {
                right += 1;
            }
            ans = ans.max(heights[i] * (right - left - 1));
        }

        ans
    }
}

// 单调栈，维护最大栈
mod method2 {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        // [0, heights, 0]
        let mut heights: Vec<i32> = heights;
        heights.insert(0, 0);
        heights.push(0);

        let mut stack: Vec<usize> = vec![];
        let mut ans: i32 = 0;

        for i in 0..heights.len() {
            // 只要栈不为空，且当前元素小于栈顶元素
            while !stack.is_empty() && heights[i] < heights[*stack.last().unwrap()] {
                let top: usize = stack.pop().unwrap();
                let right: usize = i;
                let left: usize = *stack.last().unwrap();
                ans = ans.max(heights[top] * (right - left - 1) as i32);
            }
            stack.push(i);
        }

        ans
    }
}

#[test]
fn test() {
    use method2::largest_rectangle_area;

    assert_eq!(largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
    assert_eq!(largest_rectangle_area(vec![2, 4]), 4);
}
