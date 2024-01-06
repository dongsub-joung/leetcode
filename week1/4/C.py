# https://leetcode.com/problems/kth-largest-element-in-an-array/
import heapq

class Solution:
    # https://walkccc.me/LeetCode/problems/0215/#__tabbed_2_3
    # heap
    def findKthLargest(nums: list[int], k: int) -> int:
        min_heap= []

        for num in nums:
            heapq.heappush(min_heap, num)

            if len(min_heap) > k:
                heapq.heappop(min_heap)

        return min_heap[0]
    
    # quick sort