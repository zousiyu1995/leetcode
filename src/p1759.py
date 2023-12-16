import unittest


# 找出由相同字符构成的子串，其长度为k，则该子串中同质子字符串个数为1+2+...+k个
# 找到全部由相同字符构成的子串，即可得到答案
# 每一个由相同字符构成的子串可以看成一组，典型的分组循环
class Solution:
    @staticmethod
    def countHomogenous(s: str) -> int:
        ans = 0
        i = 0
        while i < len(s):
            start = i
            i += 1
            while i < len(s) and s[i] == s[i - 1]:
                i += 1
            ans += (i - start) * (1 + (i - start)) // 2

        return ans % (10**9 + 7)


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.countHomogenous("abbcccaa"), 13)
        self.assertEqual(Solution.countHomogenous("xy"), 2)
        self.assertEqual(Solution.countHomogenous("zzzzz"), 15)


if __name__ == "__main__":
    unittest.main()
