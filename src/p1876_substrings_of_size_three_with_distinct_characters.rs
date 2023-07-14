#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s1: String = String::from("xyzzaz");
        assert_eq!(count_good_substrings(s1), 1);

        let s2: String = String::from("aababcabc");
        assert_eq!(count_good_substrings(s2), 4);
    }
}

pub fn count_good_substrings(s: String) -> i32 {
    let mut count: i32 = 0;
    let s_vec: Vec<char> = s.chars().collect();

    if s_vec.len() < 3 {
        return 0;
    }

    for i in 0..(s_vec.len() - 2) {
        if s_vec[i] != s_vec[i + 1] && s_vec[i] != s_vec[i + 2] && s_vec[i + 1] != s_vec[i + 2] {
            count += 1;
        }
    }
    count
}
