#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(reverse_vowels("hello".to_string()), "holle".to_string());
        assert_eq!(
            reverse_vowels("leetcode".to_string()),
            "leotcede".to_string()
        );
    }
}

pub fn reverse_vowels(s: String) -> String {
    let mut s: Vec<char> = s.chars().collect();
    let vowels: [char; 10] = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let mut l: usize = 0;
    let mut r: usize = s.len() - 1;

    while l < r {
        // 左右指针指向的元素都是元音，交换
        if vowels.contains(&s[l]) && vowels.contains(&s[r]) {
            s.swap(l, r);
            l += 1;
            r -= 1;
        }
        // 只要左或右指针指向的元素不是元音，移动指针
        else {
            if !vowels.contains(&s[l]) {
                l += 1;
            }
            if !vowels.contains(&s[r]) {
                r -= 1;
            }
        }
    }

    s.iter().collect()
}
