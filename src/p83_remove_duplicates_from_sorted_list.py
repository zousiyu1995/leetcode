from util.list_node import ListNode
from typing import Optional


class Solution:
    @staticmethod
    def deleteDuplicates(head: Optional[ListNode]) -> Optional[ListNode]:
        if head is None:
            return head
        # 初始化cur指向头节点
        cur = head
        # 只要cur有下一个值
        while cur.next:
            # 如果cur的下一个值和cur相同，删除
            if cur.next.val == cur.val:
                cur.next = cur.next.next
            # 如果不一样，移动cur
            else:
                cur = cur.next

        return head


def main():
    # head1 = ListNode(1)
    # head1.insert(1)
    # head1.insert(2)
    # print(head1)
    # Solution.deleteDuplicates(head1)
    # print(head1)

    head2 = ListNode(1)
    head2.insert(1)
    head2.insert(2)
    head2.insert(3)
    head2.insert(3)
    print(head2)
    Solution.deleteDuplicates(head2)
    print(head2)


if __name__ == "__main__":
    main()
