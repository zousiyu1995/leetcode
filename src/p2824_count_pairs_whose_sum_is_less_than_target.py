from typing import List


class Solution:
    def countPairs(nums: List[int], target: int) -> int:
        # 先排序
        nums.sort()

        # 双指针
        l = 0
        r = len(nums) - 1
        ans = 0
        while l < r:
            # 证明以l为左端点，以[l+1, r]中任意一个数为右端点的数对都满足要求
            # 此时更新答案，移动左端点
            if nums[l] + nums[r] < target:
                ans += r - l
                l += 1
            # 证明r太大了，为了缩小l+r，移动右端点
            else:
                r -= 1

        return ans


def test():
    nums1 = [-1, 1, 2, 3, 1]
    print(Solution.countPairs(nums1, 2))
    assert Solution.countPairs(nums1, 2) == 3

    nums2 = [-6, 2, 5, -2, -7, -1, 3]
    print(Solution.countPairs(nums2, -2))
    assert Solution.countPairs(nums2, -2) == 10


if __name__ == "__main__":
    test()
