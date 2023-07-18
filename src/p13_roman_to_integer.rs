#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s1: String = String::from("III");
        assert_eq!(roman_to_int(s1), 3);

        let s2: String = String::from("IV");
        assert_eq!(roman_to_int(s2), 4);

        let s3: String = String::from("IX");
        assert_eq!(roman_to_int(s3), 9);
    }
}

use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
    let map: HashMap<char, i32> = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);
    let s: Vec<char> = s.chars().collect();

    let mut ans: i32 = 0;
    // [0, len - 1)
    for i in 0..(s.len() - 1) {
        // if curent >= next
        if map.get(&s[i]) >= map.get(&s[i + 1]) {
            ans += map.get(&s[i]).unwrap();
        }
        // else current < next
        else {
            ans -= map.get(&s[i]).unwrap();
        }
    }
    ans += map.get(&s[s.len() - 1]).unwrap();

    ans
}
