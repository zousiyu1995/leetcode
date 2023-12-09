import unittest
from typing import List


# 找至多包含两种元素的最长子串，返回其最大长度
# 维护窗口内的哈希表，哈希表长度<=2，返回窗口的最大长度
class Solution:
    @staticmethod
    def totalFruit(fruits: List[int]) -> int:
        ans = 0
        cnt = dict()
        l = 0
        for r in range(len(fruits)):
            cnt[fruits[r]] = cnt.get(fruits[r], 0) + 1
            # 不满足要求，移动窗口
            while len(cnt) > 2:
                cnt[fruits[l]] -= 1
                if cnt[fruits[l]] == 0:
                    cnt.pop(fruits[l])
                l += 1
            ans = max(ans, r - l + 1)

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        # self.assertEqual(Solution.totalFruit([1, 2, 1]), 3)
        # self.assertEqual(Solution.totalFruit([0, 1, 2, 2]), 3)
        # self.assertEqual(Solution.totalFruit([1, 2, 3, 2, 2]), 4)
        self.assertEqual(Solution.totalFruit([3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4]), 5)


if __name__ == "__main__":
    unittest.main()
