# https://leetcode.cn/problems/find-the-longest-semi-repetitive-substring/description/

import unittest
from typing import List


# 找最大的窗口
# 最多两个连续字符相同
class Solution:
    @staticmethod
    def longestSemiRepetitiveSubstring(s: str) -> int:
        ans: int = 1
        same: int = 0
        l: int = 0

        for r in range(1, len(s)):
            # 统计相邻相同的情况
            same += s[r] == s[r - 1]
            # 如果窗口不满足条件
            if same > 1:
                l += 1
                # 将一对相同的字符移出窗口
                while s[l] != s[l - 1]:
                    l += 1
                same = 1
            ans = max(ans, r - l + 1)

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.longestSemiRepetitiveSubstring("52233"), 4)
        self.assertEqual(Solution.longestSemiRepetitiveSubstring("5494"), 4)
        self.assertEqual(Solution.longestSemiRepetitiveSubstring("1111111"), 2)


if __name__ == "__main__":
    unittest.main()
