#[test]
fn test() {
    use method1::find_repeated_dna_sequences;

    let s1: String = "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_string();
    let ans1: Vec<String> = ["AAAAACCCCC", "CCCCCAAAAA"].map(|s| s.to_string()).to_vec();
    assert_eq!(find_repeated_dna_sequences(s1), ans1);

    let s2: String = "AAAAAAAAAAAAA".to_string();
    let ans2: Vec<String> = ["AAAAAAAAAA"].map(|s| s.to_string()).to_vec();
    assert_eq!(find_repeated_dna_sequences(s2), ans2);

    let s3: String = "A".to_string();
    let ans3: Vec<String> = vec![];
    assert_eq!(find_repeated_dna_sequences(s3), ans3);

    let s4: String = "AAAAAAAAAAA".to_string();
    let ans4: Vec<String> = ["AAAAAAAAAA"].map(|s| s.to_string()).to_vec();
    assert_eq!(find_repeated_dna_sequences(s4), ans4);
}

// 哈希表，滑移窗口
// 用滑移窗口获取每一个子串，用哈希表计数子串出现的次数
mod method1 {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        const LEN: usize = 10;
        if s.len() < LEN {
            return vec![];
        }

        use std::collections::HashMap;

        let mut cnt: HashMap<String, i32> = HashMap::new();
        let mut ans: Vec<String> = vec![];

        for i in 0..=(s.len() - LEN) {
            let sub_s: String = s[i..(i + LEN)].to_string();
            *cnt.entry(sub_s.clone()).or_insert(0) += 1;
            // 只要子串计数 = 2，遍历完整个字符串，其计数应该 >=2，就是答案
            if cnt.get(&sub_s).unwrap() == &2 {
                ans.push(sub_s);
            }
            // 如果计数 >= 2，并且不在 ans 中
            // if cnt.get(&sub_s).unwrap() >= &2 && !ans.contains(&sub_s) {
            //     ans.push(sub_s);
            // }
        }

        ans
    }
}
