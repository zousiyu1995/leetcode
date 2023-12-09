import unittest
from typing import List


# https://leetcode.cn/problems/frequency-of-the-most-frequent-element/description/
# 先排序，在数组上维护一个窗口，窗口的和为sum_w
# 假定窗口里面的元素全部增加到nums[r]的大小，此时窗口的和为nums[r] * (r - l + 1)
# 由于只能增加k次，nums[r] * (r -l + 1) - sum_必须小于等于k，此时更新答案；否则不满足要求，移动窗口
class Solution:
    @staticmethod
    def maxFrequency(nums: List[int], k: int) -> int:
        nums.sort()

        ans = 0
        sum_w = 0
        l = 0
        for r in range(len(nums)):
            sum_w += nums[r]
            while nums[r] * (r - l + 1) - sum_w > k:
                sum_w -= nums[l]
                l += 1
            ans = max(ans, r - l + 1)

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.maxFrequency([1, 2, 4], 5), 3)
        self.assertEqual(Solution.maxFrequency([1, 4, 8, 13], 5), 2)
        self.assertEqual(Solution.maxFrequency([3, 9, 6], 2), 1)


if __name__ == "__main__":
    unittest.main()
