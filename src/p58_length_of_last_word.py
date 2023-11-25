import unittest


# 可以直接调用库函数，不过这样就忽视了原理
class Solution1:
    def lengthOfLastWord(s: str) -> int:
        return len(s.split()[-1])


# 自己实现
# 从字符串后面往前遍历
class Solution2:
    def lengthOfLastWord(s: str) -> int:
        i = len(s) - 1
        # 先过滤尾部空格
        while i >= 0 and s[i] == " ":
            i -= 1
        if i < 0:
            return 0

        # 对尾部字符计数
        ans = 0
        while i >= 0 and s[i] != " ":
            i -= 1
            ans += 1

        return ans


class Test(unittest.TestCase):
    def test(self):
        s1 = "Hello World"
        s2 = "   fly me   to   the moon  "
        s3 = "luffy is still joyboy"
        self.assertEqual(Solution2.lengthOfLastWord(s1), 5)
        self.assertEqual(Solution2.lengthOfLastWord(s2), 4)
        self.assertEqual(Solution2.lengthOfLastWord(s3), 6)


if __name__ == "__main__":
    unittest.main()
