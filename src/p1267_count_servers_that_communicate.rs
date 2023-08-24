#[test]
fn test() {
    assert_eq!(count_servers(vec![vec![1, 0], vec![0, 1]]), 0);
    assert_eq!(count_servers(vec![vec![1, 0], vec![1, 1]]), 3);
    assert_eq!(
        count_servers(vec![
            vec![1, 1, 0, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 1]
        ]),
        4
    );
}

pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
    let m: usize = grid.len();
    let n: usize = grid[0].len();
    let mut row: Vec<i32> = vec![0; m];
    let mut col: Vec<i32> = vec![0; n];
    let mut ans: i32 = 0;

    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 {
                row[i] += 1;
                col[j] += 1;
            }
        }
    }

    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 && (row[i] >= 2 || col[j] >= 2) {
                ans += 1;
            }
        }
    }

    ans
}
