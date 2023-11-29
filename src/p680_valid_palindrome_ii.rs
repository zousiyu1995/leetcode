#[test]
fn test() {
    assert_eq!(valid_palindrome("aba".to_string()), true);
    assert_eq!(valid_palindrome("abca".to_string()), true);
    assert_eq!(valid_palindrome("abc".to_string()), false);
}

// 用双指针判断是否是回文串，这不必多说
// 这里的技巧是，如果发现不是回文串时，就删掉l或者r，也就是去判断[l, r-1]或者[l+1, r]是不是回文串
pub fn valid_palindrome(s: String) -> bool {
    // 发现l!=r时，选择l+1或者r+1
    // 此后如果再次出现l!=r，false；如果不出现l!=r，true

    let mut l: usize = 0;
    let mut r: usize = s.len() - 1;
    while l < r {
        // 判断原串是不是回文，如果是
        if &s[l..l + 1] == &s[r..r + 1] {
            l += 1;
            r -= 1;
        }
        // 如果原字符串不是回文串，删除一个字符，即判断[l, r-1]和[l+1, r]是不是回文
        else {
            return is_palindrome((&s[l..=r - 1]).to_owned())
                || is_palindrome((&s[l + 1..=r]).to_owned());
        }
    }

    true
}

pub fn is_palindrome(s: String) -> bool {
    let mut l: usize = 0;
    let mut r: usize = s.len() - 1;

    while l < r {
        if &s[l..l + 1] == &s[r..r + 1] {
            l += 1;
            r -= 1;
        } else {
            return false;
        }
    }

    true
}
