#https://leetcode.com/problems/longest-substring-without-repeating-characters/

class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        used= {}
        max_len= start= 0
        for idx, c in enumerate(s):
            if c in used and start <= used[c]:
                start= used[c] + 1
            else:
                max_len= max(max_len, idx - start + 1)

            used[c]= idx
            
        return max_len