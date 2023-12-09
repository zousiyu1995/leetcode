import unittest
from typing import List


# 遇0清零，遇1加1
class Solution:
    @staticmethod
    def findMaxConsecutiveOnes(nums: List[int]) -> int:
        ans = 0
        cnt = 0
        for i in range(len(nums)):
            if nums[i] == 0:
                cnt = 0
            else:
                cnt += 1
            ans = max(ans, cnt)

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.findMaxConsecutiveOnes([1, 1, 0, 1, 1, 1]), 3)
        self.assertEqual(Solution.findMaxConsecutiveOnes([1, 0, 1, 1, 0, 1]), 2)


if __name__ == "__main__":
    unittest.main()
