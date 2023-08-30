# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

    def __str__(self) -> str:
        vals = []
        tail = self
        while tail.next:
            vals.append(str(tail.val))
            tail = tail.next
        vals.append(str(tail.val))

        return " -> ".join(vals)

    def insert(self, val):
        node = ListNode(val, None)
        tail = self
        # 遍历节点直到最后一个
        while tail.next:
            tail = tail.next
        tail.next = node
