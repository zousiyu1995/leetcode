import unittest


# 维护一个哈希表，该哈希表需要满足如下要求，
# 所有字符出现的次数 - 出现次数最多的字符的出现次数 <= k
# 这个要求比较难想到
class Solution:
    @staticmethod
    def characterReplacement(s: str, k: int) -> int:
        ans = 0
        cnt = dict()

        l = 0
        for r in range(len(s)):
            cnt[s[r]] = cnt.get(s[r], 0) + 1
            # 如果哈希表不满足要求，移动窗口
            if sum(cnt.values()) - max(cnt.values()) > k:
                cnt[s[l]] -= 1
                if cnt[s[l]] == 0:
                    cnt.pop(s[l])
                l += 1
            # 如果哈希表满足要求，代表子串符合要求，更新最大值
            ans = max(ans, r - l + 1)

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.characterReplacement("ABAB", 2), 4)
        self.assertEqual(Solution.characterReplacement("AABABBA", 1), 4)


if __name__ == "__main__":
    unittest.main()
