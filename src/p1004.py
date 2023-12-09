import unittest
from typing import List


# 维护窗口，要求窗口长度w <= 1的个数 + k
class Solution:
    @staticmethod
    def longestOnes(nums: List[int], k: int) -> int:
        ans = 0

        l = 0
        cnt = 0
        for r in range(len(nums)):
            if nums[r] == 1:
                cnt += 1
            # 如果窗口不满足要求，移动窗口
            if cnt + k < (r - l + 1):
                if nums[l] == 1:
                    cnt -= 1
                l += 1

            ans = max(ans, r - l + 1)

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.longestOnes([1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2), 6)
        self.assertEqual(
            Solution.longestOnes(
                [0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1], 3
            ),
            10,
        )


if __name__ == "__main__":
    unittest.main()
