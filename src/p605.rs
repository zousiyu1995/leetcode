#[test]
fn test() {
    use method2::can_place_flowers;

    let flowerbed1: Vec<i32> = vec![1, 0, 0, 0, 1];
    assert_eq!(can_place_flowers(flowerbed1, 1), true);

    let flowerbed2: Vec<i32> = vec![1, 0, 0, 0, 1];
    assert_eq!(can_place_flowers(flowerbed2, 2), false);

    let flowerbed3: Vec<i32> = vec![1, 0, 1, 0, 1, 0, 1];
    assert_eq!(can_place_flowers(flowerbed3, 0), true);

    let flowerbed4: Vec<i32> = vec![1, 0, 0, 0, 1, 0, 1];
    assert_eq!(can_place_flowers(flowerbed4, 1), true);

    let flowerbed5: Vec<i32> = vec![1, 0, 0, 0, 1, 0, 0];
    assert_eq!(can_place_flowers(flowerbed5, 2), true);
}

// https://leetcode.cn/problems/can-place-flowers/description/

// 跳格子
mod method1 {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut n: i32 = n;

        // 跳格子
        let mut idx: usize = 0;
        while idx < flowerbed.len() && n > 0 {
            // 当前位置是1，下下个位置才有可能种花
            if flowerbed[idx] == 1 {
                idx += 2;
            }
            // 当前位置是0，下一个是0或者下一个越界，可以种花
            else if idx == (flowerbed.len() - 1) || flowerbed[idx + 1] == 0 {
                n -= 1;
                idx += 2;
            }
            // 当前位置是0，下一个是1，下下下个位置才有可能种花
            else {
                idx += 3;
            }
        }

        n == 0
    }
}

// 从左到右遍历数组，如果flowerbed[i-1]、flowerbed[i]和flowerbed[i+1]均为0，种花
mod method2 {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut n: i32 = n;
        let mut flowerbed: Vec<i32> = flowerbed;
        // 在数组开头和末尾各插入一个0
        flowerbed.insert(0, 0);
        flowerbed.push(0);

        for i in 1..flowerbed.len() - 1 {
            // 先判断n>0，表示花没有种完时再去判断是否能种花
            if n > 0 && flowerbed[i - 1] == 0 && flowerbed[i] == 0 && flowerbed[i + 1] == 0 {
                flowerbed[i] = 1; // 种花
                n -= 1
            }
        }

        n <= 0
    }
}
