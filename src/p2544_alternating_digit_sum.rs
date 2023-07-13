#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(alternate_digit_sum(521), 4);
        assert_eq!(alternate_digit_sum(111), 1);
        assert_eq!(alternate_digit_sum(886996), 0);
    }
}

// pub fn alternate_digit_sum(n: i32) -> i32 {
//     let mut idx: i32 = 0;
//     let mut sum: i32 = 0;

//     for c in n.to_string().chars() {
//         idx += 1;
//         if idx % 2 == 1 {
//             sum += c.to_digit(10).unwrap() as i32;
//         } else {
//             sum += -(c.to_digit(10).unwrap() as i32);
//         }
//     }

//     sum
// }

pub fn alternate_digit_sum(n: i32) -> i32 {
    n.to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i32)
        .enumerate()
        .fold(
            0,
            |acc, (idx, x)| if idx % 2 == 0 { acc + x } else { acc - x },
        )
}
