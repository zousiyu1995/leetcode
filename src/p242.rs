#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s1: String = String::from("anagram");
        let t1: String = String::from("nagaram");
        assert_eq!(is_anagram(s1, t1), true);

        let s2: String = String::from("rat");
        let t2: String = String::from("car");
        assert_eq!(is_anagram(s2, t2), false);
    }
}

// based on map
use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut map_s: HashMap<char, i32> = HashMap::new();
    let mut map_t: HashMap<char, i32> = HashMap::new();
    s.chars().zip(t.chars()).for_each(|(c_s, c_t)| {
        map_s
            .entry(c_s)
            .and_modify(|x: &mut i32| *x += 1)
            .or_insert(0);
        map_t
            .entry(c_t)
            .and_modify(|x: &mut i32| *x += 1)
            .or_insert(0);
    });

    map_s == map_t
}

// base on array
// pub fn is_anagram(s: String, t: String) -> bool {
//     if s.len() != t.len() {
//         return false;
//     }

//     let mut hash: [i32; 26] = [0; 26];
//     for c_s in s.chars() {
//         hash[c_s as usize - 'a' as usize] += 1;
//     }
//     for c_t in t.chars() {
//         hash[c_t as usize - 'a' as usize] -= 1;
//     }
//     // 统计hash表中不等于0的位置的数量
//     hash.iter().filter(|&&x| x != 0).count() == 0
// }
