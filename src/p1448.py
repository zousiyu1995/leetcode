# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def goodNodes(self, root: TreeNode, mx=-inf) -> int:
        if root is None:
            return 0

        # 递归左子树，获取左子树的好节点个数
        left = self.goodNodes(root.left, max(mx, root.val))
        # 递归右子树，获取右子树的好节点个数
        right = self.goodNodes(root.right, max(mx, root.val))

        return left + right + (mx <= root.val)
