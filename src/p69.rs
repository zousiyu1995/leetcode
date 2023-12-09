#[cfg(test)]
mod tests {
    use crate::p69_my_sqrt::my_sqrt;

    #[test]
    fn test() {
        // assert_eq!(my_sqrt(0), 0);
        // assert_eq!(my_sqrt(4), 2);
        assert_eq!(my_sqrt(8), 2);
    }
}

pub fn my_sqrt(x: i32) -> i32 {
    if x == 0 || x == 1 {
        return x;
    }

    let (mut l, mut r) = (0, x);
    // [l..r)
    while l < r {
        let m = l + (r - l) / 2;
        // 用除法防止溢出
        if m < (x / m) {
            l = m + 1;
        } else if m > (x / m) {
            r = m;
        } else {
            return m;
        }
    }

    (l - 1) as i32
}
