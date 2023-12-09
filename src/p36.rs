#[test]
fn test() {
    let board1: Vec<Vec<char>> = [
        ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ]
    .map(|arr| arr.to_vec())
    .to_vec();
    assert_eq!(method2::is_valid_sudoku(board1), true);

    let board2: Vec<Vec<char>> = [
        ['8', '3', '.', '.', '7', '.', '.', '.', '.'],
        ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ]
    .map(|arr| arr.to_vec())
    .to_vec();
    assert_eq!(method2::is_valid_sudoku(board2), false);
}

mod method1 {
    // 数独是 9x9 的
    // 数字 1-9 在每一行只能出现一次
    // 数字 1-9 在每一列只能出现一次
    // 数字 1-9 在每一个以粗实线分隔的 3x3 宫内只能出现一次
    // 3x3 九宫格的索引是难点
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // 用 hashmap 检查每一行、列、9宫格是否有效
        let mut map_row: [[i32; 9]; 9] = [[0; 9]; 9];
        let mut map_col: [[i32; 9]; 9] = [[0; 9]; 9];
        let mut map_subsudoku: [[[i32; 9]; 3]; 3] = [[[0; 9]; 3]; 3];

        // i 行 j 列
        for i in 0..9usize {
            for j in 0..9usize {
                let ch: char = board[i][j];
                if ch != '.' {
                    let num: usize = ch.to_digit(10).unwrap() as usize;
                    // num: [1, 2, ..., 9]
                    // idx: [0, 1, ..., 8]
                    map_row[i][num - 1] += 1;
                    map_col[j][num - 1] += 1;
                    map_subsudoku[i / 3][j / 3][num - 1] += 1;
                    if map_row[i][num - 1] > 1
                        || map_row[j][num - 1] > 1
                        || map_subsudoku[i / 3][j / 3][num - 1] > 1
                    {
                        return false;
                    }
                }
            }
        }

        true
    }
}

mod method2 {
    // 用 hashset
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        use std::collections::HashSet;

        let mut rows: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut cols: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut subsudokus: Vec<HashSet<char>> = vec![HashSet::new(); 9];

        for i in 0..9usize {
            for j in 0..9usize {
                let ch = board[i][j];
                // 只处理数字
                if ch != '.' {
                    // 调用 insert 时，如果值不存在，返回 true
                    if rows[i].insert(ch)
                        && cols[j].insert(ch)
                        && subsudokus[i / 3 * 3 + j / 3].insert(ch)
                    {
                        continue;
                    } else {
                        return false;
                    }
                }
            }
        }

        true
    }
}
