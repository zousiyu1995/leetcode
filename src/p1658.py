import unittest
from typing import List


# 实际上就是求一个滑动窗口的最大长度，要求窗口之外的元素的和等于x
class Solution:
    @staticmethod
    def minOperations(nums: List[int], x: int) -> int:
        sum_nums = sum(nums)
        len_w = -1  # 窗口长度
        sum_w = 0  # 窗口的和
        l = 0
        for r in range(len(nums)):
            sum_w += nums[r]
            # 窗口之外的元素的和<x，继续扩大窗口是没有意义的，移动窗口
            while l <= r and sum_nums - sum_w < x:
                sum_w -= nums[l]
                l += 1
            # 这里窗口之外的元素的和可能>=x，但只有=x才更新答案
            if sum_nums - sum_w == x:
                len_w = max(len_w, r - l + 1)

        return -1 if len_w < 0 else len(nums) - len_w


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.minOperations([1, 1, 4, 2, 3], 5), 2)
        self.assertEqual(Solution.minOperations([5, 6, 7, 8, 9], 4), -1)
        self.assertEqual(Solution.minOperations([3, 2, 20, 1, 1, 3], 10), 5)


if __name__ == "__main__":
    unittest.main()
