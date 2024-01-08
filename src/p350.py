# https://leetcode.cn/problems/intersection-of-two-arrays-ii/description/
import unittest
from typing import List


# 哈希
class Solution:
    @staticmethod
    def intersect(nums1: List[int], nums2: List[int]) -> List[int]:
        from collections import Counter

        if len(nums1) > len(nums2):
            return Solution.intersect(nums2, nums1)

        # 对较短的数组构建哈希表，让num1成为较短的数组
        cnt = Counter(nums1)

        ans: list[int] = []
        for num in nums2:
            if cnt.get(num, 0) > 0:
                ans.append(num)
                cnt[num] -= 1

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.intersect([1, 2, 2, 1], [2, 2]), [2, 2])
        self.assertEqual(Solution.intersect([4, 9, 5], [9, 4, 9, 8, 4]), [9, 4])


if __name__ == "__main__":
    unittest.main()
