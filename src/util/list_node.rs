// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// 自定义方法
impl ListNode {
    // l: 1->2
    // l.push(3): 1->2->3
    pub fn push(&mut self, val: i32) {
        // 新建node
        let node: Option<Box<ListNode>> = Some(Box::new(ListNode::new(val)));

        // self就是head节点
        // 遍历到tail节点，如果tail.next=None，设置tail.next=node
        let mut tail: &mut ListNode = self;
        while let Some(ref mut next) = tail.next {
            tail = next;
        }
        tail.next = node;
    }

    pub fn print(&self) {
        let mut vals: Vec<String> = vec![];
        let mut tail: &ListNode = self;
        // 遍历节点，只要不为空
        while let Some(ref next) = tail.next {
            vals.push(format!("{}", &tail.val));
            tail = next;
        }
        vals.push(format!("{}", tail.val));

        println!("{}", vals.join(" -> "));
    }
}
// 从数组创建链表
// impl<const N: usize> From<[i32; N]> for ListNode {
//     // from [1, 2, 3, 4, 5]
//     //  to   1->2->3->4->5
//     fn from(array: [i32; N]) -> Self {}
// }
