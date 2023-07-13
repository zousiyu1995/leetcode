#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let v1: Vec<i32> = vec![-4, -1, 0, 3, 10];
        assert_eq!(sorted_squares(v1), vec![0, 1, 9, 16, 100]);

        let v2: Vec<i32> = vec![-7, -3, 2, 3, 11];
        assert_eq!(sorted_squares(v2), vec![4, 9, 9, 49, 121]);

        let v3: Vec<i32> = vec![-3, -2, -1, 1, 2, 3];
        assert_eq!(sorted_squares(v3), vec![1, 1, 4, 4, 9, 9]);
    }
}

pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    // 新建结果数组
    let input_length: usize = nums.len();
    let mut result: Vec<i32> = vec![0; input_length];
    let mut k: usize = input_length; // result指针

    let mut left: usize = 0; // nums左指针
    let mut right: usize = input_length - 1; // nums右指针

    while left <= right {
        k -= 1;
        if nums[left].pow(2) >= nums[right].pow(2) {
            result[k] = nums[left].pow(2);
            left += 1;
        } else {
            result[k] = nums[right].pow(2);
            right -= 1;
            // 注意：最极限的情况，left=0，right=0，如果此时进入这个else，right-1=-1，usize类型越界了。
            // 所以，应该交给if处理，left+1肯定不越界。或者指针干脆用i32类型。
        }
    }

    result
}
