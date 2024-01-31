# https://leetcode.cn/problems/replace-the-substring-for-balanced-string/description/


import unittest


# 如果在待替换子串之外的任意字符的出现次数都不超过 m 次，那么可以通过替换使s成为平衡字符串
class Solution:
    @staticmethod
    def balancedString(s: str) -> int:
        from collections import Counter
        from math import inf

        cnt = Counter(s)
        m: int = len(s) // 4

        # 四个字符出现次数均为m次，已经符合要求了
        if all(cnt[x] == m for x in "QWER"):
            return 0

        ans: int = len(s)
        l = 0
        for r in range(len(s)):
            cnt[s[r]] -= 1  # cnt维护了窗口之外的字符的出现次数，进入窗口，-1
            while all(cnt[x] <= m for x in "QWER"):
                ans = min(ans, r - l + 1)
                cnt[s[l]] += 1  # 移出窗口，+1
                l += 1

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.balancedString("QWER"), 0)
        self.assertEqual(Solution.balancedString("QQWE"), 1)
        self.assertEqual(Solution.balancedString("QQQW"), 2)
        self.assertEqual(Solution.balancedString("QQQQ"), 3)


if __name__ == "__main__":
    unittest.main()
