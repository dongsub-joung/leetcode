# https://leetcode.com/problems/3sum/

# My code
class Solution:
    def threeSum(nums: list[int]) -> list[list[int]]:
        sums= []
        for i in range(len(nums)):
            for j in range(1, len(nums)):
                for k in range(2,len(nums)):
                    if nums[i] + nums[j] + nums[k] == 0:
                        sums.append([i,j,k])
        
        return sums

# book   
class Solution:
    def threeSum2(nums: list[int]) -> list[list[int]]:
        results=[]
        nums.sort()

        for i in range(len(nums) - 2):
            if i>0 and nums[i] == nums[i-1]:
                continue
            l, r= i+1, len(nums) -1

            while l < r:
                sum= nums[i] + nums[l] + nums[r]
                if sum<0:
                    l+=1

result= Solution.threeSum2([-1,0,1,2,-1,-4])
# Output: [[-1,-1,2],[-1,0,1]]
print(result)
