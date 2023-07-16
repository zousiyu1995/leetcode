#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s1 = String::from("abcdefg");
        assert_eq!(reverse_str(s1, 2), String::from("bacdfeg"));

        let s2: String = String::from("abcd");
        assert_eq!(reverse_str(s2, 2), String::from("bacd"));
    }
}

pub fn reverse_str(s: String, k: i32) -> String {
    // 转换成字符数组
    let mut s: Vec<char> = s.chars().collect();

    // i = l + k - 1;
    // j = l + 2 * k - 1;
    // 反转 [l, i]
    // [x, x, x, x, x, x, x, x, x]
    //  ^     ^        ^        ^
    //  l     i        j        r
    // [x, x, x] _
    //  ^  ^  ^  ^
    //  l  i  r  j
    // 反转 [l, r]
    // [x, x] _, _, _, _
    //  ^     ^        ^
    //  l  r  i        j
    let mut l: usize = 0;
    let r: usize = s.len() - 1;
    let k: usize = k as usize;
    while l <= r {
        let i: usize = l + k - 1;
        if i <= r {
            reverse(&mut s, l, i);
        } else {
            reverse(&mut s, l, r);
        }
        l += 2 * k; // 下一次循环的起始点
    }

    let s: String = s.iter().collect();
    s
}

pub fn reverse(s: &mut Vec<char>, mut l: usize, mut r: usize) {
    // 反转字符数组的指定区间
    // [x, x, x, x, x]
    //  ^           ^
    //  l ->     <- r
    while l < r {
        s.swap(l, r);
        l += 1;
        r -= 1;
    }
}
