from typing import Optional
from util.list_node import ListNode


class Solution:
    @staticmethod
    def reverseList(head: Optional[ListNode]) -> Optional[ListNode]:
        # p206，反转链表
        pre = None
        cur = head
        while cur:
            nxt = cur.next
            cur.next = pre
            pre = cur
            cur = nxt
        return pre

    @staticmethod
    def middleNode(head: Optional[ListNode]) -> Optional[ListNode]:
        # p876，链表的中间节点
        slow = head
        fast = head
        # 每次慢指针走1步，快指针走2步，最终慢指针就是答案
        while fast and fast.next:
            slow = slow.next
            fast = fast.next.next

        return slow

    @staticmethod
    def reorderList(head: Optional[ListNode]) -> None:
        """
        Do not return anything, modify head in-place instead.
        """
        # 首先找到中间节点，反转[mid, tail]这一段链表
        # 将[head, mid)、[mid, tail]视为两段链表
        # 然后依次从两个链表里面各拿一个节点，合并成新链表
        mid = Solution.middleNode(head)
        head2 = Solution.reverseList(mid)
        while head2.next:
            nxt = head.next
            nxt2 = head2.next
            head.next = head2
            head2.next = nxt
            head = nxt
            head2 = nxt2


def main():
    head = ListNode(1, None)
    head.insert(2)
    head.insert(3)
    head.insert(4)

    Solution.reorderList(head)
    print(head)


if __name__ == "__main__":
    main()
