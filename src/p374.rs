/**
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

// 不能单独运行，API由leetcode提供
impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let mut l: i32 = 1;
        let mut r: i32 = n;
        // [l, r]
        while l < r {
            let m: i32 = l + (r - l) / 2;
            if guess(m) == 1 {
                l = m + 1;
            } else if guess(m) == -1 {
                r = m - 1;
            } else {
                return m;
            }
        }

        l
    }
}
