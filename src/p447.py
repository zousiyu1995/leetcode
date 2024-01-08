# https://leetcode.cn/problems/number-of-boomerangs/description/
import unittest
from typing import List


# 枚举i、j、k三个点即可，但是时间复杂度是O(N^3)
# 用哈希表优化成O(N^2)
class Solution:
    @staticmethod
    def numberOfBoomerangs(points: List[List[int]]) -> int:
        from collections import Counter

        ans = 0
        # 枚举i
        for x_i, y_i in points:
            cnt = Counter()
            # 枚举j
            for x_j, y_j in points:
                dist: int = (x_i - x_j) ** 2 + (y_i - y_j) ** 2
                cnt[dist] += 1
            # 更新答案，从能的组成答案的数中挑两个
            for vals in cnt.values():
                ans += vals * (vals - 1)

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.numberOfBoomerangs([[0, 0], [1, 0], [2, 0]]), 2)
        self.assertEqual(Solution.numberOfBoomerangs([[1, 1], [2, 2], [3, 3]]), 2)
        self.assertEqual(Solution.numberOfBoomerangs([[1, 1]]), 0)


if __name__ == "__main__":
    unittest.main()
