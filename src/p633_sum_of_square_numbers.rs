#[test]
fn test() {
    // assert_eq!(judge_square_sum(5), true);
    // assert_eq!(judge_square_sum(3), false);
    assert_eq!(judge_square_sum(4), true);
}

// 双指针
pub fn judge_square_sum(c: i32) -> bool {
    let c: i64 = c as i64;
    let mut l: i64 = 0;
    let mut r: i64 = (c as f64).sqrt() as i64; // 设置r=c会超时

    while l <= r {
        let tmp: i64 = l * l + r * r;
        if tmp > c {
            r -= 1;
        } else if tmp < c {
            l += 1;
        } else {
            return true;
        }
    }

    false
}
