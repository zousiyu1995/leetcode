#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let h1: String = "sadbutsad".to_string();
        let n1: String = "sad".to_string();
        assert_eq!(str_str(h1, n1), 0);

        let h2: String = "leetcode".to_string();
        let n2: String = "leeto".to_string();
        assert_eq!(str_str(h2, n2), -1);
    }
}

pub fn str_str(haystack: String, needle: String) -> i32 {
    match haystack.find(&needle) {
        Some(v) => v as i32,
        None => -1,
    }
}

// TODO: KMP
