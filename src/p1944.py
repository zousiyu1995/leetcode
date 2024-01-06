# https://leetcode.cn/problems/number-of-visible-people-in-a-queue/description/
import unittest
from typing import List


# 单调栈，倒序遍历
# 维护最小栈
class Solution:
    @staticmethod
    def canSeePersonsCount(heights: List[int]) -> List[int]:
        n: int = len(heights)
        ans: list[int] = [0] * n
        stack: list[int] = []

        for i in range(n - 1, -1, -1):
            # 只要栈不为空，且当前元素>栈顶元素
            while stack and stack[-1] < heights[i]:
                stack.pop()
                ans[i] += 1
            if stack:
                ans[i] += 1
            stack.append(heights[i])

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(
            Solution.canSeePersonsCount([10, 6, 8, 5, 11, 9]), [3, 1, 2, 1, 1, 0]
        )
        self.assertEqual(Solution.canSeePersonsCount([5, 1, 2, 3, 10]), [4, 1, 1, 1, 0])


if __name__ == "__main__":
    unittest.main()
