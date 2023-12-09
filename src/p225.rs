#[test]
fn test() {
    let mut my_stack: MyStack = MyStack::new();
    dbg!(
        my_stack.push(1),
        my_stack.push(2),
        my_stack.top(),   // 返回 2
        my_stack.pop(),   // 返回 2
        my_stack.empty(), // 返回 false
    );
}

use std::collections::VecDeque;
struct MyStack {
    // 用一个队列模拟栈
    stack: VecDeque<i32>,
}

impl MyStack {
    fn new() -> Self {
        Self {
            stack: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        // 题目要求只能用 push_back
        self.stack.push_back(x);
        // 栈顶元素在 back，而题目要求只能用 pop_front
        // 所以依次把 front 的元素移到 back，暴露栈顶元素
        for _ in 0..self.stack.len() - 1 {
            let x: i32 = self.stack.pop_front().unwrap();
            self.stack.push_back(x);
        }
    }

    fn pop(&mut self) -> i32 {
        self.stack.pop_front().unwrap()
    }

    fn top(&self) -> i32 {
        *self.stack.front().unwrap()
    }

    fn empty(&self) -> bool {
        self.stack.is_empty()
    }
}
