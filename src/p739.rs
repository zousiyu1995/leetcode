// https://leetcode.cn/problems/daily-temperatures/description/

// 暴力法，会超时
#[allow(unused)]
mod method1 {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![0; temperatures.len()];
        for i in 0..temperatures.len() {
            for j in (i + 1)..temperatures.len() {
                // 找到下一个更高温度
                if temperatures[j] > temperatures[i] {
                    ans[i] = (j - i) as i32;
                    break;
                }
            }
        }

        ans
    }
}

// 单调栈，在栈中存放元素的索引
mod method2 {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<usize> = vec![];
        let mut ans: Vec<i32> = vec![0; temperatures.len()];

        for cur in 0..temperatures.len() {
            // 只要栈不为空，且温度>栈顶温度，说明栈顶元素遇到了下一个更大元素，添加答案，出栈
            while !stack.is_empty() && temperatures[cur] > temperatures[*stack.last().unwrap()] {
                let top: usize = stack.pop().unwrap();
                ans[top] = (cur - top) as i32;
            }
            // 温度<=栈顶元素，入栈
            stack.push(cur);
        }

        ans
    }
}

#[test]
fn test() {
    use self::method2::daily_temperatures;

    assert_eq!(
        daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
        vec![1, 1, 4, 2, 1, 1, 0, 0]
    );
    assert_eq!(daily_temperatures(vec![30, 40, 50, 60]), vec![1, 1, 1, 0]);
    assert_eq!(daily_temperatures(vec![30, 60, 90]), vec![1, 1, 0]);
}
