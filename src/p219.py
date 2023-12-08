import unittest
from typing import List


# 用哈希表判断是否遇到重复，重复时检查下标距离
class Solution1:
    @staticmethod
    def containsNearbyDuplicate(nums: List[int], k: int) -> bool:
        # 元素 => 下标
        hashmap = dict()

        for i in range(len(nums)):
            num = nums[i]
            if num in hashmap and i - hashmap[num] <= k:
                return True
            # 无论num在不在hashmap中，都更新下标
            hashmap[num] = i

        return False


# 题意是窗口内是否包括重复数字，维护窗口的哈希表就行了
class Solution2:
    @staticmethod
    def containsNearbyDuplicate(nums: List[int], k: int) -> bool:
        # set记录窗口内出现过的元素
        hashset = set()

        w = k + 1  # 最大窗口是k+1，如果比这个窗口大，移除元素
        for i in range(len(nums)):
            if i >= w:
                hashset.remove(nums[i - w])

            # 新进的元素是否在set中，是，返回True；否，加入set
            if nums[i] in hashset:
                return True
            else:
                hashset.add(nums[i])

        return False


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution2.containsNearbyDuplicate([1, 2, 3, 1], 3), True)
        self.assertEqual(Solution2.containsNearbyDuplicate([1, 0, 1, 1], 1), True)
        self.assertEqual(
            Solution2.containsNearbyDuplicate([1, 2, 3, 1, 2, 3], 2), False
        )
        self.assertEqual(Solution2.containsNearbyDuplicate([99, 99], 2), True)


if __name__ == "__main__":
    unittest.main()
