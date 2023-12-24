import unittest
from itertools import accumulate
from typing import List


# 暴力
# 维护一个长度为k的滑动窗口，[i, i + k)
# 对于每一个以nums[i]开头的窗口，将窗口内所有元素减去nums[i]来保证nums[i]变为0。然后移动窗口
# 需要注意的是，并不能保证最后一个窗口内的全部元素为0。因此需要检查最后一个窗口
class Solution1:
    @staticmethod
    def checkArray(nums: List[int], k: int) -> bool:
        n = len(nums)
        # [i, i + k)
        for i in range(n - k + 1):
            if nums[i] < 0:
                return False
            elif nums[i] > 0:
                x = nums[i]
                # 修改窗口内的全部元素
                for j in range(i, i + k):
                    nums[j] -= x

        # 检查最后一个窗口，[n - k, n)
        for i in range(n - k, n):
            if nums[i] != 0:
                return False

        return True


# 差分。如果原数组全部元素为0，等价于差分数组的全部元素为0
class Solution2:
    @staticmethod
    def checkArray(nums: List[int], k: int) -> bool:
        n = len(nums)

        # 构建差分数组
        diff = [0] * (n + 1)
        diff[0] = nums[0]
        for i in range(1, n):
            diff[i] = nums[i] - nums[i - 1]
        diff[n] = -nums[n - 1]

        # 操作差分数组
        for i in range(n - k + 1):
            if diff[i] > 0:
                diff[i + k] += diff[i]
                diff[i] = 0

        # 检查差分数组
        for i in diff:
            if i != 0:
                return False

        return True


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution2.checkArray([2, 2, 3, 1, 1, 0], 3), True)
        self.assertEqual(Solution2.checkArray([1, 3, 1, 1], 2), False)


if __name__ == "__main__":
    unittest.main()
