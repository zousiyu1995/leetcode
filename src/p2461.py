import unittest
from typing import List


# 滑移窗口维护子数组
# 哈希表判断是否各不相同
class Solution:
    @staticmethod
    def maximumSubarraySum(nums: List[int], k: int) -> int:
        n = len(nums)

        ans = 0
        cnt = dict()  # 窗口的哈希表
        sum = 0  # 窗口的和

        # 枚举窗口右端点
        for i in range(n):
            cnt[nums[i]] = cnt.get(nums[i], 0) + 1
            sum += nums[i]
            if i >= k:
                cnt[nums[i - k]] -= 1
                sum -= nums[i - k]
                # 及时删除个数为0的键
                if cnt[nums[i - k]] == 0:
                    cnt.pop(nums[i - k])
            # 子数组元素各不相同
            if len(cnt) == k:
                ans = max(ans, sum)

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.maximumSubarraySum([1, 5, 4, 2, 9, 9, 9], 3), 15)
        self.assertEqual(Solution.maximumSubarraySum([4, 4, 4], 3), 0)
        self.assertEqual(Solution.maximumSubarraySum([1, 1, 1, 7, 8, 9], 3), 24)


if __name__ == "__main__":
    unittest.main()
