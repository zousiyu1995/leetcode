from util.list_node import ListNode
from typing import Optional


class Solution:
    @staticmethod
    def deleteDuplicates(head: Optional[ListNode]) -> Optional[ListNode]:
        dummy = ListNode(next=head)
        cur = dummy
        # 每次循环看下两个节点的值是否一样
        while cur.next and cur.next.next:
            val = cur.next.val
            # 如果一样，不断删除节点
            if cur.next.next.val == val:
                while cur.next and cur.next.val == val:
                    cur.next = cur.next.next  # 删除cur.next节点
            else:
                cur = cur.next

        return dummy.next


def main():
    head = ListNode(1)
    head.insert(2)
    head.insert(3)
    head.insert(3)
    head.insert(4)
    head.insert(4)
    head.insert(5)
    print(head)

    Solution.deleteDuplicates(head)
    print(head)


if __name__ == "__main__":
    main()
