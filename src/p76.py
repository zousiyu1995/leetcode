from typing import Dict
import unittest


# 暴力法，滑移窗口，超时，能通过264/267测试用例
# 窗口从小到大依次去找s中满足要求的子串
class Solution1:
    @staticmethod
    def minWindow(s: str, t: str) -> str:
        # s比t短，肯定找不到
        if len(s) < len(t):
            return ""

        # 构建t的哈希表，t中的字符数
        map_t: Dict[str, int] = dict()
        for c in t:
            map_t[c] = map_t.get(c, 0) + 1

        def find(len_w) -> str:
            # 维护s上的窗口
            for i in range(0, len(s) - len_w + 1):
                # 构建子串的哈希表
                sub_s: str = s[i : i + len_w]
                map_s: Dict[str, int] = dict()
                for c in sub_s:
                    # t中包含c，并且map_s中c的计数比map_t小
                    if c in map_t.keys() and map_s.get(c, 0) < map_t[c]:
                        map_s[c] = map_s.get(c, 0) + 1
                # 如何哈希表相等，找到了
                if map_s == map_t:
                    return sub_s
            # 没找到，返回空串
            return ""

        # 只要没找到，增加窗口大小
        for i in range(len(t), len(s) + 1):
            ans: str = find(i)
            if ans != "":
                return ans

        # 找完了也没找到，返回空串
        return ""


# 优化一下方法1
# 不断扩大左边界，直到包括t中所有字符
# 然后缩小右边界，保证子串最短
class Solution2:
    @staticmethod
    def minWindow(s: str, t: str) -> str:
        if len(s) < len(t):
            return ""

        # t中字符的哈希表，需要匹配的字符
        need = dict()
        for c in t:
            need[c] = need.get(c, 0) + 1

        l = 0
        r = 0
        count = dict()  # 已匹配的满足条件的字符的哈希表
        mch = 0  # 已经匹配了多少种字符
        start = 0  # 满足条件的子串的起点
        length = len(s) + 1  # 满足条件的子串的长度，初始化为比字符串s的长度大
        # 延长窗口直到满足条件
        while r < len(s):
            c = s[r]
            r += 1
            if c in need:
                count[c] = count.get(c, 0) + 1
                if count[c] == need[c]:
                    mch += 1
            # 满足条件之后缩短窗口
            while mch == len(need):
                # 更新最短子串
                if r - l < length:
                    start = l
                    length = r - l
                c = s[l]
                l += 1
                # 如果从窗口左边删除的元素是所需要的
                if c in need:
                    if count[c] == need[c]:
                        mch -= 1
                    count[c] -= 1

        # 如果length等于初始长度，证明没匹配任何字符，返回空串
        return "" if length > len(s) else s[start : start + length]


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution2.minWindow("ADOBECODEBANC", "ABC"), "BANC")
        self.assertEqual(Solution2.minWindow("a", "a"), "a")
        self.assertEqual(Solution2.minWindow("a", "aa"), "")
        self.assertEqual(Solution2.minWindow("a", "b"), "")
        self.assertEqual(
            Solution2.minWindow("aaaaaaaaaaaabbbbbcdd", "abcdd"), "abbbbbcdd"
        )


if __name__ == "__main__":
    unittest.main()
