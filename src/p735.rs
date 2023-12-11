pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
    let mut stack: Vec<i32> = vec![];

    for asteroid in asteroids {
        // 判断当前行星是否存活的flag
        let mut alive: bool = true;

        // 只要当前行星存活，栈不为空，栈顶元素>0，当前元素<0，才可能出栈
        while alive && !stack.is_empty() && *stack.last().unwrap() > 0 && asteroid < 0 {
            let a: i32 = *stack.last().unwrap();
            let b: i32 = asteroid.abs();
            alive = a < b; // 栈顶元素的绝对值小，当前行星才会存活
            if a <= b {
                stack.pop();
            }
        }

        // 只有在当前行星存活的情况下才入栈
        if alive {
            stack.push(asteroid);
        }
    }

    stack
}

#[test]
fn test() {
    assert_eq!(asteroid_collision(vec![5, 10, -5]), vec![5, 10]);
    assert_eq!(asteroid_collision(vec![8, -8]), vec![]);
    assert_eq!(asteroid_collision(vec![10, 2, -5]), vec![10]);
    assert_eq!(asteroid_collision(vec![-2, -1, 1, 2]), vec![-2, -1, 1, 2]);
}
