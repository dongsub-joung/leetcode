# https://leetcode.com/problems/top-k-frequent-elements/

from collections import Counter
from heapq import heappush, heappop

class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        freqs= Counter(nums)
        freqs_heap= []

        for f in freqs:
            heappush(freqs_heap, (-freqs[f], f))

        topk= list()
        for _ in range(k):
            topk.append(heappop(freqs_heap)[1])
        
        return topk