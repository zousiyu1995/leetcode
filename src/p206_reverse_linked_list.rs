#[test]
fn test() {
    let head = Some(Box::new(ListNode::from([1, 2, 3, 4, 5])));
    head.as_ref().unwrap().print();

    let rev_head = reverse_list(head);
    rev_head.as_ref().unwrap().print();
}

use crate::util::ListNode;

// 难点在于处理Rust的Option
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut pre: Option<Box<ListNode>> = None;
    let mut cur: Option<Box<ListNode>> = head;

    // 原始链表
    // None    1 -> 2 -> 3 -> 4 -> 5 -> None
    //  ^      ^    ^
    // pre    cur  next

    // 第一步，只要cur不为空，反转cur的指向
    // None <- 1 -> 2 -> 3 -> 4 -> 5 -> None
    //  ^      ^    ^
    // pre    cur  next

    // 第二步，移动pre、cur
    // None <- 1 -> 2 -> 3 -> 4 -> 5 -> None
    //         ^    ^    ^
    //        pre  cur  next

    // cur为空，反转结束时
    // None <- 1 <- 2 <- 3 <- 4 <- 5    None
    //                             ^     ^
    //                            pre   cur

    // TODO
    // 只要当前节点不为空，就反转
    // while cur.is_some() {
    //     let next = cur.unwrap().next;
    // }

    while let Some(mut node) = cur {
        cur = node.next.take();
        node.next = pre; // 反转指向
        pre = Some(node);
    }

    pre
}
