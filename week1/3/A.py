# https://leetcode.com/problems/jewels-and-stones/

class Solution:
    def numJewelsInStones(jewels: str, stones: str) -> int:
        # Allocate dictionary
        freqs= {}
        count= 0

        for stone in stones:
            if stone not in freqs:
                # allocate value 1
                freqs[stone]= 1
            else:
                # stone in freqs
                freqs[stone] += 1
        
        for jewel in jewels:
            if jewel in freqs:
                count+= freqs[jewel]

        return count
    
result= Solution.numJewelsInStones("aA", "aAAbbbb")
# Output: 3
print(result)