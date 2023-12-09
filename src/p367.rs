#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(is_perfect_square(16), true);
        assert_eq!(is_perfect_square(14), false);
        assert_eq!(is_perfect_square(5), false);
    }
}

pub fn is_perfect_square(num: i32) -> bool {
    if num == 1 {
        return true;
    }

    let num: i64 = num as i64;
    let mut l: i64 = 1;
    let mut r: i64 = num;
    // [l, r)
    while l < r {
        let m = l + (r - l) / 2;
        if m * m < num {
            l = m + 1;
        } else if m * m > num {
            r = m;
        } else {
            return true;
        }
    }

    false
}
