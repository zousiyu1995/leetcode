#[test]
fn test() {
    use method2::find_the_longest_balanced_substring;

    assert_eq!(
        find_the_longest_balanced_substring("01000111".to_string()),
        6
    );
    assert_eq!(find_the_longest_balanced_substring("00111".to_string()), 4);
    assert_eq!(find_the_longest_balanced_substring("111".to_string()), 0);
}

// 就是个模拟题，感觉method1更清晰
// 遍历字符串，依次统计连续0的数量、连续1的数量。两者中最小的数量*2就是平衡字符串的长度
// 不断更新该长度既能得到答案
#[allow(unused)]
mod method1 {
    pub fn find_the_longest_balanced_substring(s: String) -> i32 {
        let mut ans: i32 = 0;
        let mut i: usize = 0;

        while i < s.len() {
            let mut zero: i32 = 0;
            let mut one: i32 = 0;
            // 统计连续0的长度
            while i < s.len() && &s[i..i + 1] == "0" {
                zero += 1;
                i += 1;
            }
            // 统计连续1的长度
            while i < s.len() && &s[i..i + 1] == "1" {
                one += 1;
                i += 1;
            }
            let len: i32 = zero.min(one) * 2;
            ans = ans.max(len);
        }

        ans
    }
}

// 和method1思路一样，换for循环
#[allow(unused)]
mod method2 {
    pub fn find_the_longest_balanced_substring(s: String) -> i32 {
        let mut ans: i32 = 0;
        let mut zero: i32 = 0;
        let mut one: i32 = 0;

        for i in 0..s.len() {
            // 如果当前字符是1
            if &s[i..i + 1] == "1" {
                one += 1;
                ans = ans.max(2 * zero.min(one));
            }
            // 如果当前字符是第一个字符。或者，如果当前字符是0，且是第一个0
            else if i == 0 || &s[i - 1..i] == "1" {
                zero = 1;
                one = 0;
            }
            // 如果当前字符是0
            else {
                zero += 1;
            }
        }

        ans
    }
}
