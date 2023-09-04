from util.list_node import ListNode


class Solution:
    def deleteNode(self, node):
        # 把下个节点的值复制过来，然后当前节点指向下下个节点
        node.val = node.next.val
        node.next = node.next.next


def main():
    head = ListNode(4, None)
    head.insert(5)
    head.insert(1)
    head.insert(9)
    print(head)


if __name__ == "__main__":
    main()
