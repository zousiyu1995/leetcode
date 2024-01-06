# https://leetcode.cn/problems/maximum-width-ramp/description/
import unittest
from typing import List


# 单调栈，维护最大栈
class Solution:
    @staticmethod
    def maxWidthRamp(nums: List[int]) -> int:
        n: int = len(nums)
        ans: int = 0
        stack: list[int] = []  # 存下标

        # 正序遍历数组，生成一个单调递减序列，让左边界尽可能小
        for i in range(n):
            # 如果栈是空的，或者当前元素<栈顶元素
            if not stack or nums[i] < nums[stack[-1]]:
                stack.append(i)

        # 逆序遍历数组
        for j in range(n - 1, -1, -1):
            # 如果栈不是空的，且当前元素>=栈顶元素，代表能形成一个坡，并且这个坡肯定是以左边界为坡底的、最宽的坡
            while stack and nums[j] >= nums[stack[-1]]:
                ans = max(ans, j - stack[-1])
                stack.pop()

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.maxWidthRamp([6, 0, 8, 2, 1, 5]), 4)
        self.assertEqual(Solution.maxWidthRamp([9, 8, 1, 0, 1, 9, 4, 0, 4, 1]), 7)


if __name__ == "__main__":
    unittest.main()
