#[test]
fn test() {
    let v1: Vec<i32> = vec![-1, 0, 1, 2, -1, -4];
    assert_eq!(three_sum(v1), vec![vec![-1, -1, 2], vec![-1, 0, 1]]);

    let v2: Vec<i32> = vec![0, 1, 1];
    let ans2: Vec<Vec<i32>> = vec![];
    assert_eq!(three_sum(v2), ans2);

    let v3: Vec<i32> = vec![0, 0, 0];
    assert_eq!(three_sum(v3), vec![vec![0, 0, 0]])
}

// 对撞双指针
// 枚举 i，问题变为两数之和
// [x, x, x, x, x] 数组
//  ^  ^        ^
//  i  l        r  指针位置
// i最多只能到数组的倒数第三个位置
pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    // 不够三个数
    if nums.len() < 3 {
        return vec![vec![]];
    }

    // sort
    nums.sort();

    let mut ans: Vec<Vec<i32>> = vec![];
    for i in 0..(nums.len() - 2) {
        // i 的去重。只要当前枚举的 i 和上一个 i 相同，跳过这个 i，因为上一次已经处理过这个 i 了
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        // 双指针
        let mut l: usize = i + 1;
        let mut r: usize = nums.len() - 1;
        while l < r {
            let sum: i32 = nums[i] + nums[l] + nums[r];
            if sum > 0 {
                // 太大了，缩右边界
                r -= 1;
            } else if sum < 0 {
                // 太小了，缩左边界
                l += 1;
            } else {
                // =0，找到了，先去重
                while l < r && nums[l] == nums[l + 1] {
                    l += 1;
                }
                while l < r && nums[r] == nums[r - 1] {
                    r -= 1;
                }
                ans.push(vec![nums[i], nums[l], nums[r]]);
                // 同时缩左右边界，找下一个
                l += 1;
                r -= 1;
            }
        }
    }

    ans
}
