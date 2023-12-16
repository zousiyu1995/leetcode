import unittest


# 分组循环
# 保证相同连续字符<=2
class Solution:
    @staticmethod
    def makeFancyString(s: str) -> str:
        ans = ""
        i = 0
        while i < len(s):
            cnt = 1
            ans += s[i]
            i += 1
            # 碰到连续字符，更新计数
            while i < len(s) and s[i] == s[i - 1]:
                cnt += 1
                # 更新ans的前提是连续字符<=2
                if cnt <= 2:
                    ans += s[i]
                i += 1

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.makeFancyString("leeetcode"), "leetcode")
        self.assertEqual(Solution.makeFancyString("aaabaaaa"), "aabaa")
        self.assertEqual(Solution.makeFancyString("aab"), "aab")


if __name__ == "__main__":
    unittest.main()
