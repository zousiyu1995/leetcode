#[test]
fn test() {
    let mut stack: MinStack = MinStack::new();
    stack.push(-2);
    stack.push(0);
    stack.push(-3);
    println!("{}", stack.get_min());
    stack.pop();
    println!("{}", stack.top());
    println!("{}", stack.get_min());
}

struct MinStack {
    stack: Vec<(i32, i32)>, // (element, min)
}

// 要求top是O(1)的操作，因此应该在构建栈的时候计算出最小值
// 使用元组跟踪最小值，在栈内储存元组(val, min)，val是压入栈的元素，min是当前入栈时计算出的最小值
// 实际上相当于用了两个栈，一个栈维护值，另一个维护最小值
#[allow(unused)]
impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: vec![(-1, i32::MAX)], // 提前压入一个值
        }
    }

    fn push(&mut self, val: i32) {
        // 每次和栈顶的最小值比，维护最小值
        self.stack
            .push((val, val.min(self.stack.last().unwrap().1)));
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        self.stack.last().unwrap().0
    }

    fn get_min(&self) -> i32 {
        self.stack.last().unwrap().1
    }
}
