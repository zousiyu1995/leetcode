#[cfg(test)]
mod test {
    use super::can_change;

    #[test]
    fn test1() {
        let start: String = "_L__R__R_".to_string();
        let target: String = "L______RR".to_string();
        assert_eq!(can_change(start, target), true);
    }

    #[test]
    fn test2() {
        let start: String = "R_L_".to_string();
        let target: String = "__LR".to_string();
        assert_eq!(can_change(start, target), false);
    }

    #[test]
    fn test3() {
        let start: String = "_R".to_string();
        let target: String = "R_".to_string();
        assert_eq!(can_change(start, target), false);
    }
}

// 条件1：两字符串长度相等
// 条件2：字符串仅由'_'、'L'和'R'组成
// 字符串、双指针
pub fn can_change(start: String, target: String) -> bool {
    // 观察可知，去掉'_'后，两字符串必须相等，否则无法从start得到target
    if start.replace("_", "") != target.replace("_", "") {
        return false;
    }

    // 观察可知只存在两种非法情况
    // 1. L不能右移，如下情况非法
    // start:  [x, L, x, ...]
    // target: [x, x, L, ...]
    // 2. R不能左移，如下情况非法
    // start:  [..., x, x, R]
    // target: [..., x, R, x]
    let mut j: usize = 0;
    let target: Vec<char> = target.chars().collect(); // 坑：rust不能索引字符串
    for (i, ch_i) in start.chars().enumerate() {
        // 迭代start直到遇到非'_'
        if ch_i == '_' {
            continue;
        }
        // 迭代target直到遇到非'_'
        while target[j] == '_' {
            j += 1;
        }
        // 处理两种非法情况
        if ch_i == 'L' && i < j {
            return false;
        }
        if ch_i == 'R' && i > j {
            return false;
        }
        j += 1;
    }

    true
}
