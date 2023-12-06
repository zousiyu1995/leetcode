from typing import List, Dict
import unittest


# https://leetcode.cn/problems/substring-with-concatenation-of-all-words/description/
# 哈希表+滑移窗口
# 思路很简单，但是简单的思路运行时间长，有很大的优化空间
class Solution:
    @staticmethod
    def findSubstring(s: str, words: List[str]) -> List[int]:
        # 构建words的哈希表
        map1: Dict[str, int] = dict()
        for word in words:
            map1[word] = map1.get(word, 0) + 1

        ans: List[int] = []
        len_subs: int = len(words[0])  # 需要哈希的子串的大小
        len_w: int = len(words) * len_subs  # 窗口大小
        # 维护s上的窗口，i是窗口的左端点
        for i in range(0, len(s) - len_w + 1):
            # 维护窗口内子串的哈希表，j是子串的左端点
            map2: Dict[str, int] = dict()
            for j in range(i, i + len_w, len_subs):
                sub_s: str = s[j : j + len_subs]
                map2[sub_s] = map2.get(sub_s, 0) + 1

            # 判断哈希表是否相等
            if map1 == map2:
                ans.append(i)

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(
            Solution.findSubstring("barfoothefoobarman", ["foo", "bar"]), [0, 9]
        )
        self.assertEqual(
            Solution.findSubstring(
                "wordgoodgoodgoodbestword", ["word", "good", "best", "word"]
            ),
            [],
        )
        self.assertEqual(
            Solution.findSubstring("barfoofoobarthefoobarman", ["bar", "foo", "the"]),
            [6, 9, 12],
        )
        self.assertEqual(
            Solution.findSubstring(
                "wordgoodgoodgoodbestword", ["word", "good", "best", "good"]
            ),
            [8],
        )


if __name__ == "__main__":
    unittest.main()
