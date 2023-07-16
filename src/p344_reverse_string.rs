#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut s1: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];
        reverse_string(&mut s1);
        assert_eq!(s1, vec!['o', 'l', 'l', 'e', 'h']);

        let mut s2: Vec<char> = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        reverse_string(&mut s2);
        assert_eq!(s2, vec!['h', 'a', 'n', 'n', 'a', 'H']);
    }
}

pub fn reverse_string(s: &mut Vec<char>) {
    // [x, x, x, x, x]
    //  ^           ^
    //  l ->     <- r
    let mut l: usize = 0;
    let mut r: usize = s.len() - 1;
    while l < r {
        s.swap(l, r);
        l += 1;
        r -= 1;
    }
}
