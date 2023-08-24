#[test]
fn test() {
    use method2::h_index;

    // assert_eq!(h_index(vec![0, 1, 3, 5, 6]), 3);
    // assert_eq!(h_index(vec![1, 2, 100]), 2);
    assert_eq!(h_index(vec![0]), 0);
}

// 与p274类似，但是citations是有序的
// 时间复杂度：O(n)
#[allow(unused)]
mod method1 {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        // 引用次数>h_index，h_index+1
        let mut h_index: i32 = 0;
        for &citation in citations.iter().rev() {
            if citation > h_index {
                h_index += 1;
            }
        }

        h_index
    }
}

// 二分查找可以降低时间复杂度
// 时间复杂度：O(log(n))
// 设数组长度为n
// 每次二分可以知道，mid右侧的论文（共n-mid篇）至少被引用了citations[mid]次
// 如果citations[mid]>=n-mid，说明可以往右扩展多包括几篇文献，h index可能会更大，移动right
// 否则，移动left
// 二分的关键还是边界问题
#[allow(unused)]
mod method2 {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n: usize = citations.len();
        let mut left: usize = 0;
        let mut right: usize = n;
        // [left, right)
        while left < right {
            let mid: usize = left + (right - left) / 2;
            if citations[mid] >= (n - mid) as i32 {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        (n - left) as i32
    }
}
