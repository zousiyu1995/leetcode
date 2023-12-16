import unittest


# 维护窗口
class Solution1:
    @staticmethod
    def maxPower(s: str) -> int:
        ans = 0
        l = 0
        for r in range(len(s)):
            if s[r] != s[l]:
                l = r
            ans = max(ans, r - l + 1)

        return ans


# 分组循环
class Solution2:
    @staticmethod
    def maxPower(s: str) -> int:
        ans = 0
        i = 0
        # 维护起点
        while i < len(s):
            start = i
            i += 1
            # 维护终点
            while i < len(s) and s[i] == s[i - 1]:
                i += 1
            ans = max(ans, i - start)

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution2.maxPower("leetcode"), 2)
        self.assertEqual(Solution2.maxPower("abbcccddddeeeeedcba"), 5)
        self.assertEqual(Solution2.maxPower("ccbccbb"), 2)


if __name__ == "__main__":
    unittest.main()
