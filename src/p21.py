from typing import Optional
from util.list_node import ListNode


class Solution:
    def mergeTwoLists(
        list1: Optional[ListNode], list2: Optional[ListNode]
    ) -> Optional[ListNode]:
        dummy = ListNode()  # 新链表的头节点，是哑节点
        cur = dummy  # 新链表的当前节点

        # 遍历list1和list2，哪个小就把哪个加入新链表
        while list1 is not None and list2 is not None:
            if list1.val < list2.val:
                cur.next = list1
                list1 = list1.next
            else:
                cur.next = list2
                list2 = list2.next
            cur = cur.next

        # list1和list2可能不等长，拼接剩下的链表
        cur.next = list1 if list1 is not None else list2

        return dummy.next


def test():
    l1 = ListNode(1)
    l1.insert(2)
    l1.insert(4)

    l2 = ListNode(1)
    l2.insert(3)
    l2.insert(4)

    print(Solution.mergeTwoLists(l1, l2))


if __name__ == "__main__":
    test()
