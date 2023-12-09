#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let grid1: Vec<Vec<i32>> = vec![vec![1, 2, 4], vec![3, 3, 1]];
        assert_eq!(delete_greatest_value(grid1), 8);

        let grid2: Vec<Vec<i32>> = vec![vec![10]];
        assert_eq!(delete_greatest_value(grid2), 10);
    }
}

pub fn delete_greatest_value(grid: Vec<Vec<i32>>) -> i32 {
    let mut grid: Vec<Vec<i32>> = grid;
    // sort
    for row in grid.iter_mut() {
        row.sort();
    }
    // m行，n列
    let m: usize = grid.len();
    let n: usize = grid[0].len();
    let mut ans: i32 = 0;
    // 遍历列
    for n_i in 0..n {
        let mut max: i32 = 0;
        // 遍历行
        for m_i in 0..m {
            max = grid[m_i][n_i].max(max);
        }
        ans += max;
    }

    ans
}
