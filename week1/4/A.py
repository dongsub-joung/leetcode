# https://leetcode.com/problems/maximum-product-of-two-elements-in-an-array/

# my code
class Solution:
    def maxProduct(nums: list[int]) -> int:
        _max=0
        for i, _ in enumerate(nums):
            for j in range(1, len(nums)):
                condition= (nums[i]-1) * (nums[j]-1)
                _max= max(_max, condition)
        return _max

# https://github.com/doocs/leetcode/blob/main/solution/1400-1499/1464.Maximum%20Product%20of%20Two%20Elements%20in%20an%20Array/README_EN.md
    def maxProduct(self, nums: List[int]) -> int:
        answer= 0
        for i, a in enumerate(nums):
            for b in nums[i+1:]:
                answer= max(answer, (a-1) * (b-1))
        return answer

result= Solution.maxProduct([3,7])
result2= Solution.maxProduct([1,5,4,5])
# 12
print(result)
# 16
print(result2)