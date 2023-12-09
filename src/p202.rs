#[test]
fn test() {
    use method1::is_happy;

    assert_eq!(is_happy(19), true);
    assert_eq!(is_happy(2), false);
}

// 数是正整数
mod method1 {
    pub fn is_happy(n: i32) -> bool {
        use std::collections::HashSet;

        // 计算下一个数
        fn get_next_num(mut n: i32) -> i32 {
            let mut sum: i32 = 0;
            while n != 0 {
                let num: i32 = n % 10;
                n = n / 10;
                sum += num.pow(2);
            }
            sum
        }

        let mut n: i32 = n;
        let mut set: HashSet<i32> = HashSet::new();
        // 要点是，数字可能会陷入循环
        // 如果 set 中已经有这个数了，代表陷入循环了
        // 除此之外，都会得到 1
        while n != 1 && !set.contains(&n) {
            set.insert(n);
            n = get_next_num(n);
        }

        n == 1
    }
}
