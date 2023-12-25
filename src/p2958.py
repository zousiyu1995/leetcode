import unittest
from typing import List


# 题意：要求子数组中每一个元素的出现次数<=k，求满足该要求的子数组的最大长度
class Solution:
    @staticmethod
    def maxSubarrayLength(nums: List[int], k: int) -> int:
        from collections import Counter

        ans = 0
        cnt = Counter()
        l = 0
        for r in range(len(nums)):
            cnt[nums[r]] += 1
            # 只要窗口不满足要求，移动窗口
            while cnt[nums[r]] > k:
                cnt[nums[l]] -= 1
                l += 1
            ans = max(ans, r - l + 1)

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.maxSubarrayLength([1, 2, 3, 1, 2, 3, 1, 2], 2), 6)
        self.assertEqual(Solution.maxSubarrayLength([1, 2, 1, 2, 1, 2, 1, 2], 1), 2)
        self.assertEqual(Solution.maxSubarrayLength([5, 5, 5, 5, 5, 5, 5], 4), 4)


if __name__ == "__main__":
    unittest.main()
