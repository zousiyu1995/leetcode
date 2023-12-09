#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let str1: String = "ABCABC".to_string();
        let str2: String = "ABC".to_string();
        assert_eq!(gcd_of_strings(str1, str2), "ABC".to_string());
    }
}

pub fn gcd_of_strings(str1: String, str2: String) -> String {
    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

    if format!("{}{}", &str1, &str2) != format!("{}{}", &str2, &str1) {
        return "".to_string();
    } else {
        str1[0..gcd(str1.len(), str2.len())].to_string()
    }
}
