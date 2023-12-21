import unittest
from itertools import accumulate
from typing import List


# https://leetcode.cn/problems/car-pooling/description/
# 差分数组
class Solution:
    @staticmethod
    def carPooling(trips: List[List[int]], capacity: int) -> bool:
        diff = [0] * 1001
        for num, from_, to in trips:
            diff[from_] += num
            diff[to] -= num

        for s in accumulate(diff):
            if s > capacity:
                return False

        return True


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.carPooling([[2, 1, 5], [3, 3, 7]], 4), False)
        self.assertEqual(Solution.carPooling([[2, 1, 5], [3, 3, 7]], 5), True)


if __name__ == "__main__":
    unittest.main()
