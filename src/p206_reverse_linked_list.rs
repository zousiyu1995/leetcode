#[test]
fn test() {
    let head: Option<Box<ListNode>> = Some(Box::new(ListNode::from([1, 2, 3, 4, 5])));
    head.as_ref().unwrap().print();

    let rev_head: Option<Box<ListNode>> = reverse_list(head);
    rev_head.as_ref().unwrap().print();
}

use crate::util::ListNode;

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut pre: Option<Box<ListNode>> = None;
    let mut cur: Option<Box<ListNode>> = head;

    // 第一步，只要cur不为空，暂存cur的下一个节点为next
    // None    1 -> 2 -> 3 -> 4 -> 5 -> None
    //  ^      ^    ^
    // pre    cur  next
    //
    // 第二步，反转cur的指向。注意，此时cur和next已经失去联系了
    // None <- 1    2 -> 3 -> 4 -> 5 -> None
    //  ^      ^    ^
    // pre    cur  next
    //
    // 第三步，移动pre、cur
    // None <- 1 -> 2 -> 3 -> 4 -> 5 -> None
    //         ^    ^
    //        pre  cur
    //
    // 重复以上步骤，直到cur为空，反转结束
    // None <- 1 <- 2 <- 3 <- 4 <- 5    None
    //                             ^     ^
    //                            pre   cur

    // 难点在于处理Rust的Option
    // 首先，不能直接unwarp，否则值会被消耗掉
    // 其次，获取的next节点要转移所有权
    // while cur.is_some() {
    //     let next: Option<Box<ListNode>> = cur.as_mut().unwrap().next.take();
    //     cur.as_mut().unwrap().next = pre;
    //     pre = cur;
    //     cur = next;
    // }

    // 另一种写法，可读性不强
    while let Some(mut node) = cur {
        cur = node.next.take(); // 移动cur到下一个节点
        node.next = pre; // 反转指向
        pre = Some(node); // 移动pre
    }

    pre
}
