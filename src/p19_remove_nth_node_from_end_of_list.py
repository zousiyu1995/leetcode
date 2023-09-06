from typing import Optional
from util.list_node import ListNode


# 删除倒数第n个节点
# 常规做法是先遍历链表获取长度，然后遍历链表到要被删除的节点的上一个，就可以做删除操作了
# 因为可能会删除头节点，所以必须创建dummy节点
class Solution:
    @staticmethod
    def removeNthFromEnd(head: Optional[ListNode], n: int) -> Optional[ListNode]:
        # dummy -> 1 -> 2 -> 3 -> 4 -> None
        #   ^
        # right, left：初始化左右指针指向dummy
        # 先让右指针走n步，然后左右指针一起走，左右指针的距离是n。当右指针指向末尾，左指针刚好是第n+1个节点
        dummy = ListNode(next=head)
        right = dummy
        for _ in range(n):
            right = right.next

        left = dummy
        while right.next:
            left = left.next
            right = right.next
        left.next = left.next.next

        return dummy.next


def main():
    head = ListNode(1)
    head.insert(2)
    head.insert(3)
    head.insert(4)
    head.insert(5)
    print(head)

    Solution.removeNthFromEnd(head, 2)
    print(head)


if __name__ == "__main__":
    main()
