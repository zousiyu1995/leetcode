import unittest
from typing import List


# 前缀和+哈希表
class Solution:
    @staticmethod
    def numSubarraysWithSum(nums: List[int], goal: int) -> int:
        from collections import Counter

        cnt = Counter()
        ans = 0
        sum_ = 0
        for num in nums:
            cnt[sum_] += 1
            sum_ += num
            ans += cnt[sum_ - goal]

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.numSubarraysWithSum([1, 0, 1, 0, 1], 2), 4)
        self.assertEqual(Solution.numSubarraysWithSum([0, 0, 0, 0, 0], 0), 15)


if __name__ == "__main__":
    unittest.main()
