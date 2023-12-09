#[test]
fn test() {
    assert_eq!(
        word_break(
            "leetcode".to_string(),
            vec!["leet".to_string(), "code".to_string()]
        ),
        true
    );
}

pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let mut left: usize = 0;
    for right in 0..s.len() {
        let ss = s[left..=right].to_string();
        if word_dict.contains(&ss) {
            left = right + 1;
        }
    }

    true
}
