#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let score1: Vec<Vec<i32>> = vec![vec![10, 6, 9, 1], vec![7, 5, 11, 2], vec![4, 8, 3, 15]];
        let k1: i32 = 2;
        let ans1: Vec<Vec<i32>> = vec![vec![7, 5, 11, 2], vec![10, 6, 9, 1], vec![4, 8, 3, 15]];
        assert_eq!(sort_the_students(score1, k1), ans1);
    }
}

pub fn sort_the_students(score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let mut score: Vec<Vec<i32>> = score;
    let k: usize = k as usize;
    // 按行排序
    score.sort_by(|a: &Vec<i32>, b: &Vec<i32>| b[k].partial_cmp(&a[k]).unwrap());
    score
}
