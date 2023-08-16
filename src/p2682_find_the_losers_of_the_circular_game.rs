#[test]
fn test() {
    assert_eq!(circular_game_losers(5, 2), vec![4, 5]);
    assert_eq!(circular_game_losers(4, 4), vec![2, 3, 4]);
    assert_eq!(circular_game_losers(1, 1), vec![]);
}

pub fn circular_game_losers(n: i32, k: i32) -> Vec<i32> {
    let mut map: Vec<i32> = vec![0; n as usize];
    map[0] = 1; // 第一个人有球

    let mut round: usize = 0;
    let mut current: usize = 0; // 没开始游戏时，持有球的人的编号是0

    loop {
        round += 1; // 第i轮
        current = (current + round * k as usize) % n as usize; // 当前接球的人的编号
        map[current] += 1;
        // 只要有人第二次接到球，break
        if map[current] == 2 {
            break;
        }
    }

    map.into_iter()
        .enumerate()
        .filter(|(_idx, v)| *v == 0)
        .map(|(idx, _v)| idx as i32 + 1)
        .collect()
}
