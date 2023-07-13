#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n1: i64 = 12;
        assert_eq!(maximum_even_split(n1), vec![2, 4, 6]);

        let n2: i64 = 7;
        assert_eq!(maximum_even_split(n2), vec![]);

        let n3: i64 = 28;
        assert_eq!(maximum_even_split(n3), vec![2, 4, 6, 16]);
    }
}

pub fn maximum_even_split(mut final_sum: i64) -> Vec<i64> {
    // 如果是奇数
    if final_sum % 2 > 0 {
        return vec![];
    }

    // 如果是偶数
    let mut ans: Vec<i64> = vec![];
    let mut split: i64 = 2;
    while final_sum >= split {
        ans.push(split);
        final_sum -= split;
        split += 2;
    }
    if let Some(last) = ans.last_mut() {
        *last += final_sum;
    }

    ans
}
