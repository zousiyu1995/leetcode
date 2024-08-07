import unittest
from typing import Dict, List


# 一定存在下标为i, j的两个数字满足要求，枚举j，去哈希表里找target-nums[j]
class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        map: Dict[int, int] = {}  # 数字->下标

        for j, num in enumerate(nums):
            if target - num in map:
                return [map[target - num], j]  # return [i, j]
            map[num] = j

        return []


class Test(unittest.TestCase):
    def test(self) -> None:
        solution = Solution()
        self.assertEqual(solution.twoSum([2, 7, 11, 15], 9), [0, 1])
        self.assertEqual(solution.twoSum([3, 2, 4], 6), [1, 2])
        self.assertEqual(solution.twoSum([3, 3], 6), [0, 1])


if __name__ == "__main__":
    unittest.main()
