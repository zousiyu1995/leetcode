import unittest


# 要求1：5个元音字母必须全部出现，用hashset
# 要求2：元音按字典序排列，比较新元素和窗口中元素的字典序。注意，aeiou本来就是按字典序排列的
class Solution:
    @staticmethod
    def longestBeautifulSubstring(word: str) -> int:
        ans = 0
        i = 0
        cnt = set()
        # 维护开始位置
        while i < len(word):
            start = i
            cnt.add(word[i])
            i += 1
            # 维护结束位置，保证满足要求2
            while i < len(word) and ord(word[i]) >= ord(word[i - 1]):
                cnt.add(word[i])
                i += 1
            # 如果满足要求1，更新答案
            if len(cnt) == 5:
                ans = max(ans, i - start)
            # 进入下一分组前清空hashset
            cnt.clear()

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(
            Solution.longestBeautifulSubstring("aeiaaioaaaaeiiiiouuuooaauuaeiu"), 13
        )
        self.assertEqual(Solution.longestBeautifulSubstring("aeeeiiiioooauuuaeiou"), 5)
        self.assertEqual(Solution.longestBeautifulSubstring("a"), 0)


if __name__ == "__main__":
    unittest.main()
