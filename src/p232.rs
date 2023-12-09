#[test]
fn test() {
    let mut my_queue = MyQueue::new();
    my_queue.push(1); // queue is: [1]
    dbg!(&my_queue);
    my_queue.push(2); // queue is: [1, 2] (leftmost is front of the queue)
    dbg!(&my_queue);
    println!("{}", my_queue.peek()); // return 1
    println!("{}", my_queue.pop()); // return 1, queue is [2]
    dbg!(&my_queue);
    println!("{}", my_queue.empty()); // return false
}

#[derive(Debug)]
struct MyQueue {
    in_stack: Vec<i32>,
    out_stack: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        Self {
            in_stack: vec![],
            out_stack: vec![],
        }
    }

    fn in2out(&mut self) {
        while let Some(x) = self.in_stack.pop() {
            self.out_stack.push(x);
        }
    }

    fn push(&mut self, x: i32) {
        self.in_stack.push(x);
    }

    fn pop(&mut self) -> i32 {
        // 如果 outstack 是空的，把 instack 的元素全部压入 outstack
        if self.out_stack.is_empty() {
            self.in2out();
        }
        // 如果 outstack 不是空的，正常 pop 元素
        self.out_stack.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        // 如果 outstack 是空的，把 instack 的元素全部压入 outstack
        if self.out_stack.is_empty() {
            self.in2out();
        }
        // 如果 outstack 不是空的，正常 peek 元素
        *self.out_stack.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.in_stack.is_empty() && self.out_stack.is_empty()
    }
}
