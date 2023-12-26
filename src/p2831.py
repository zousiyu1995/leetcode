import unittest
from typing import List


class Solution:
    @staticmethod
    def longestEqualSubarray(nums: List[int], k: int) -> int:
        from collections import Counter

        cnt = Counter()
        ans = 0
        l = 0
        for r in range(len(nums)):
            cnt[nums[r]] += 1
            if r - l + 1 - cnt[nums[l]] > k:
                cnt[nums[l]] -= 1
                l += 1

            ans = max(ans, cnt[nums[r]])

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.longestEqualSubarray([1, 3, 2, 3, 1, 3], 3), 3)
        self.assertEqual(Solution.longestEqualSubarray([1, 1, 2, 2, 1, 1], 2), 4)


if __name__ == "__main__":
    unittest.main()
