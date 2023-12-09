#[cfg(test)]
mod test {
    use super::letter_combinations;

    #[test]
    fn test1() {
        assert_eq!(
            letter_combinations("23".to_string()), //
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            letter_combinations("".to_string()), //
            Vec::<String>::new()
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            letter_combinations("2".to_string()), //
            vec!["a", "b", "c"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        );
    }
}

// 显然，本题是生成全排列
// 原问题：构造长为n的字符串
// 枚举一个字母后
// 子问题：构造长为n-1的字符串

pub fn letter_combinations(digits: String) -> Vec<String> {
    // 处理特殊情况
    let n: usize = digits.len();
    if n == 0 {
        return Vec::<String>::new();
    }

    use std::collections::HashMap;
    let map: HashMap<char, &str> = HashMap::from([
        ('1', ""),
        ('2', "abc"),
        ('3', "def"),
        ('4', "ghi"),
        ('5', "jkl"),
        ('6', "mno"),
        ('7', "pqrs"),
        ('8', "tuv"),
        ('9', "wxyz"),
    ]);
    let digits: Vec<char> = digits.chars().collect();

    let mut ans: Vec<String> = vec![];
    let mut path: Vec<String> = vec!["".to_string(); n];

    // dfs
    fn dfs(
        digits: &Vec<char>,        // digits
        map: &HashMap<char, &str>, //
        i: usize,                  // 第i个digit
        n: usize,                  // 递归的终点
        ans: &mut Vec<String>,     //
        path: &mut Vec<String>,    //
    ) {
        // 递归的边界条件
        if i == n {
            ans.push(path.join(""));
            return;
        }
        // 递归的非边界条件
        let s: &&str = map.get(&digits[i]).unwrap();
        for ch in s.chars() {
            path[i] = ch.to_string();
            dfs(&digits, &map, i + 1, n, ans, path);
        }
    }

    // 开始递归
    dfs(&digits, &map, 0, n, &mut ans, &mut path);

    ans
}
