import unittest


# 分治
class Solution:
    @staticmethod
    def longestSubstring(s: str, k: int) -> int:
        if len(s) < k:
            return 0

        for c in set(s):
            # 如果字符c在s中出现的次数少于k次，那么包含字符c的子串都不满足要求
            if s.count(c) < k:
                # 此时将s按c分割，分割后的每个子串t都不含c
                # 之后在子串t中继续寻找满足题意的子串
                ans = 0
                for t in s.split(c):
                    ans = max(ans, Solution.longestSubstring(t, k))
                return ans
                # 上面这段可以写成一行
                # return max(Solution.longestSubstring(t, k) for t in s.split(c))

        # 到这里说明s中的每个字符出现的次数大于k，直接返回s的长度
        return len(s)


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.longestSubstring("aaabb", 3), 3)
        self.assertEqual(Solution.longestSubstring("ababbc", 2), 5)


if __name__ == "__main__":
    unittest.main()
