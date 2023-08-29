#[test]
fn test() {
    let mut l: ListNode = ListNode::new(1);
    for i in 2..=5 {
        l.push(i);
    }
    l.print();

    let r: Option<Box<ListNode>> = reverse_list(Some(Box::new(l)));
    r.unwrap().print();
}

use super::util::list_node::ListNode;

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut pre: Option<Box<ListNode>> = None;
    let mut cur: Option<Box<ListNode>> = head;

    // None 1 -> 2 -> 3 -> 4 -> 5
    //  ^   ^
    // pre  cur
    // 只要当前节点不为空
    while let Some(mut node) = cur {
        cur = node.next.take();
        node.next = pre;
        pre = Some(node);
    }

    pre
}
