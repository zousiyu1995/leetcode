import unittest
from typing import List


# 统计窗口内的白色字符有多少个
class Solution1:
    @staticmethod
    def minimumRecolors(blocks: str, k: int) -> int:
        cnt = dict()

        # 初始化第一个窗口
        for i in range(0, k):
            cnt[blocks[i]] = cnt.get(blocks[i], 0) + 1
        ans = cnt.get("W", 0)

        # 枚举右端点，移动窗口
        for i in range(k, len(blocks)):
            # 进入窗口的元素，不一定在cnt里
            cnt[blocks[i]] = cnt.get(blocks[i], 0) + 1
            # 移出窗口的元素，肯定在cnt里
            cnt[blocks[i - k]] -= 1
            ans = min(ans, cnt.get("W", 0))

        return ans


# 方法1复杂了点，遍历一遍字符也能办到
class Solution2:
    @staticmethod
    def minimumRecolors(blocks: str, k: int) -> int:
        ans = len(blocks)
        cnt = 0
        for i in range(len(blocks)):
            # 进入窗口的元素
            if blocks[i] == "W":
                cnt += 1
            # 移出窗口的元素
            if i >= k and blocks[i - k] == "W":
                cnt -= 1
            if i >= k - 1:
                ans = min(ans, cnt)

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution2.minimumRecolors("WBBWWBBWBW", 7), 3)
        self.assertEqual(Solution2.minimumRecolors("WBWBBBW", 2), 0)
        self.assertEqual(Solution2.minimumRecolors("BWWWBB", 6), 3)
        self.assertEqual(Solution2.minimumRecolors("WWBBBWBBBBBWWBWWWB", 16), 6)


if __name__ == "__main__":
    unittest.main()
