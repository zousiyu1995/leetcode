# https://leetcode.cn/problems/max-pair-sum-in-an-array/description/

import unittest
from math import inf
from typing import List


class Solution:
    def maxSum(self, nums: List[int]) -> int:
        ans: int = -1
        # max_val定义了一个可以容纳10个元素的数组，其索引表示一个元素的最大数位
        # max_val维护了最大数位为i的最大元素
        max_val: List[int] = [-inf] * 10

        for i in nums:
            max_d: int = max(map(int, str(i)))  # 当前元素的最大数位
            ans = max(ans, i + max_val[max_d])  # 更新答案，此时已经保证了最大数位相等
            max_val[max_d] = max(max_val[max_d], i)  # 更新最大元素

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        solution = Solution()
        self.assertEqual(solution.maxSum([51, 71, 17, 24, 42]), 88)
        self.assertEqual(solution.maxSum([1, 2, 3, 4]), -1)


if __name__ == "__main__":
    unittest.main()
