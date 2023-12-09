#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let candies1: Vec<i32> = vec![2, 3, 5, 1, 3];
        assert_eq!(
            kids_with_candies(candies1, 3),
            vec![true, true, true, false, true]
        );

        let candies2: Vec<i32> = vec![4, 2, 1, 1, 2];
        assert_eq!(
            kids_with_candies(candies2, 1),
            vec![true, false, false, false, false]
        );

        let candies3: Vec<i32> = vec![12, 1, 12];
        assert_eq!(kids_with_candies(candies3, 10), vec![true, false, true]);
    }
}

pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max: i32 = *candies.iter().max().unwrap();
    candies.iter().map(|&x| x + extra_candies >= max).collect()
}
