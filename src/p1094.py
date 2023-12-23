import unittest
from typing import List


# https://leetcode.cn/problems/car-pooling/description/
# 差分数组
class Solution1:
    @staticmethod
    def carPooling(trips: List[List[int]], capacity: int) -> bool:
        from itertools import accumulate

        diff = [0] * 1001
        for num, from_, to in trips:
            diff[from_] += num
            diff[to] -= num

        for s in accumulate(diff):
            if s > capacity:
                return False

        return True


# 哈希表写法
# 除了from和to，其他位置是用不上的，因此哈希表写法相比数组写法复杂度更低
# 为了保证key是有序的，需要对dict排序。为了方便，也可以用SortedDict
class Solution2:
    @staticmethod
    def carPooling(trips: List[List[int]], capacity: int) -> bool:
        from sortedcontainers import SortedDict

        diff = SortedDict()
        for num, from_, to in trips:
            diff[from_] = diff.get(from_, 0) + num
            diff[to] = diff.get(to, 0) - num

        s = 0
        for k in diff:
            s += diff[k]
            if s > capacity:
                return False

        return True


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution2.carPooling([[2, 1, 5], [3, 3, 7]], 4), False)
        self.assertEqual(Solution2.carPooling([[2, 1, 5], [3, 3, 7]], 5), True)


if __name__ == "__main__":
    unittest.main()
