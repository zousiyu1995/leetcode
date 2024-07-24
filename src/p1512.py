# https://leetcode.cn/problems/number-of-good-pairs/description/

import unittest
from collections import defaultdict
from typing import List


# 换个思路，如果当前元素的后面有与当前元素相等的元素，等同于当前元素的前面有与当前元素相等的元素
class Solution:
    def numIdenticalPairs(self, nums: List[int]) -> int:
        ans = 0
        cnt = defaultdict(int)  # 记录当前元素的前面的元素出现的次数

        for i in nums:
            ans += cnt[i]
            cnt[i] += 1

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        solution = Solution()
        self.assertEqual(solution.numIdenticalPairs([1, 2, 3, 1, 1, 3]), 4)
        self.assertEqual(solution.numIdenticalPairs([1, 1, 1, 1]), 6)
        self.assertEqual(solution.numIdenticalPairs([1, 2, 3]), 0)


if __name__ == "__main__":
    unittest.main()
