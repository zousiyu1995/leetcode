#[test]
fn test() {
    use method2::trap;

    assert_eq!(trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    assert_eq!(trap(vec![4, 2, 0, 3, 2, 5]), 9);
}

// 前后缀分解，木桶能接多少水取决于前缀最大值和后缀最大值
// 时间复杂度 O(n)
// 空间复杂度 O(n)
mod method1 {
    #[allow(unused)]
    pub fn trap(height: Vec<i32>) -> i32 {
        let n: usize = height.len();

        // 前缀最大值
        let mut pre_max: Vec<i32> = vec![0; n];
        pre_max[0] = height[0];
        for i in 1..n {
            pre_max[i] = pre_max[i - 1].max(height[i]);
        }

        // 后缀最大值
        let mut suf_max: Vec<i32> = vec![0; n];
        suf_max[n - 1] = height[n - 1];
        for i in (0..n - 1).rev() {
            suf_max[i] = suf_max[i + 1].max(height[i]);
        }

        // 计算当前位置能接多少雨水
        let mut ans: i32 = 0;
        for i in 0..n {
            ans += pre_max[i].min(suf_max[i]) - height[i];
        }

        ans
    }
}

// 双指针
// 时间复杂度 O(n)
// 空间复杂度 O(1)
mod method2 {
    #[allow(unused)]
    pub fn trap(height: Vec<i32>) -> i32 {
        let n: usize = height.len();
        let mut ans: i32 = 0;
        let mut left: i32 = 0;
        let mut right: i32 = (n - 1) as i32;
        let mut pre_max: i32 = 0;
        let mut suf_max: i32 = 0;

        while left <= right {
            pre_max = pre_max.max(height[left as usize]);
            suf_max = suf_max.max(height[right as usize]);
            if pre_max < suf_max {
                ans += pre_max - height[left as usize];
                left += 1;
            } else {
                ans += suf_max - height[right as usize];
                right -= 1;
            }
        }

        ans
    }
}
