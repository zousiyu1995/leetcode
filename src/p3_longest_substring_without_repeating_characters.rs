#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s1: String = String::from("abcabcbb");
        assert_eq!(length_of_longest_substring(s1), 3);

        let s2: String = String::from("bbbbb");
        assert_eq!(length_of_longest_substring(s2), 1);

        let s3: String = String::from("pwwkew");
        assert_eq!(length_of_longest_substring(s3), 3);
    }
}

use std::collections::HashSet;

pub fn length_of_longest_substring(s: String) -> i32 {
    // 特殊情况
    if s.len() == 0 {
        return 0;
    }

    let s: Vec<char> = s.chars().collect();
    let mut set: HashSet<char> = HashSet::new();
    let mut l: usize = 0;
    let mut maxlen: usize = 0;
    // 使用滑移窗口，关键是窗口一定要合法。判断是否合法的关键是set数据结构
    // 只要窗口内有重复字符，窗口就是非法的，此时需要移动左指针直至窗口合法
    // 只要窗口内没有重复字符，移动右指针，扩大窗口
    // [l, r]
    for (r, c) in s.iter().enumerate() {
        // 只要窗口里有这个重复字符，从set移除该字符，移动左指针直至窗口合法
        while set.contains(c) {
            set.remove(&s[l]);
            l += 1;
        }
        // 否则，插入字符到set，进入下一次循环移动右指针扩大窗口
        set.insert(*c);
        // 比较每一个合法窗口的大小，选最大值
        maxlen = (r - l + 1).max(maxlen);
    }

    maxlen as i32
}
