import unittest
from typing import List


class Solution:
    @staticmethod
    def containsDuplicate(nums: List[int]) -> bool:
        hashset = set()
        for num in nums:
            if num in hashset:
                return True
            else:
                hashset.add(num)

        return False


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.containsDuplicate([1, 2, 3, 1]), True)
        self.assertEqual(Solution.containsDuplicate([1, 2, 3, 4]), False)
        self.assertEqual(
            Solution.containsDuplicate([1, 1, 1, 3, 3, 4, 3, 2, 4, 2]), True
        )


if __name__ == "__main__":
    unittest.main()
