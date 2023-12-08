import unittest
from typing import List


class Solution:
    @staticmethod
    def maxSum(nums: List[int], m: int, k: int) -> int:
        n = len(nums)

        ans = 0
        cnt = dict()
        sum = 0
        # 窗口大小为k，用哈希表统计是否有>=m个不同的元素
        for i in range(n):
            cnt[nums[i]] = cnt.get(nums[i], 0) + 1
            sum += nums[i]
            if i >= k:
                cnt[nums[i - k]] -= 1
                sum -= nums[i - k]
                # 如果计数为0，去掉
                if cnt[nums[i - k]] == 0:
                    cnt.pop(nums[i - k])
            if len(cnt) >= m:
                ans = max(ans, sum)

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.maxSum([2, 6, 7, 3, 1, 7], 3, 4), 18)
        self.assertEqual(Solution.maxSum([5, 9, 9, 2, 4, 5, 4], 1, 3), 23)
        self.assertEqual(Solution.maxSum([1, 2, 1, 2, 1, 2, 1], 3, 3), 0)
        self.assertEqual(Solution.maxSum([1, 2, 2], 2, 2), 3)


if __name__ == "__main__":
    unittest.main()
