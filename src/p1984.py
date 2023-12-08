import unittest
from typing import List


# 有点脑筋急转弯的意思
# 先把数组排序，那么要保证最大值和最小值之差最小，所选的k个人一定是连续的区间
# 此时，窗口的两个端点之差就是最大值和最小值之差
class Solution:
    @staticmethod
    def minimumDifference(nums: List[int], k: int) -> int:
        nums.sort()

        ans = nums[len(nums) - 1]
        # 枚举窗口左端点
        for i in range(0, len(nums) - k + 1):
            ans = min(ans, nums[i + k - 1] - nums[i])

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.minimumDifference([90], 1), 0)
        self.assertEqual(Solution.minimumDifference([9, 4, 1, 7], 2), 2)


if __name__ == "__main__":
    unittest.main()
