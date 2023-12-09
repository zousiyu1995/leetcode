#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s1: Vec<String> = vec!["flower", "flow", "flight"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(longest_common_prefix(s1), "fl".to_string());

        let s2: Vec<String> = vec!["ab", "a"].into_iter().map(|s| s.to_string()).collect();
        assert_eq!(longest_common_prefix(s2), "a".to_string());

        let s3: Vec<String> = vec!["abab", "aba", ""]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(longest_common_prefix(s3), "".to_string());

        let s4: Vec<String> = vec![""].into_iter().map(|s| s.to_string()).collect();
        assert_eq!(longest_common_prefix(s4), "".to_string());
    }
}
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    // if strs.is_empty() {
    //     return "".to_string();
    // }

    let (first, rest) = strs.split_first().unwrap();
    let mut ans: String = String::new();
    // 遍历第一个String的char
    for (idx, c_first) in first.chars().enumerate() {
        // 遍历其余的String，比较相同位置的char
        for s in rest {
            let c_rest: Vec<char> = s.chars().collect();
            // idx < s.len()时，字符串长度够；否则，长度不够，返回
            // 或者相同位置的char不一样，返回
            if idx == s.len() || c_first != c_rest[idx] {
                return ans;
            }
        }
        // 字符一样，push char
        ans.push(c_first);
    }

    ans
}
