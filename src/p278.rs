// cannot run because the API isBadVersion is defined by leetcode.
// isBadVersion(version:i32)-> bool;

pub fn first_bad_version(&self, n: i32) -> i32 {
    let mut l = 1;
    let mut r = n;
    // [l, r)
    while l < r {
        let m = l + (r - l) / 2;
        if self.isBadVersion(m) {
            r = m;
        } else {
            l = m + 1;
        }
    }

    l
}
