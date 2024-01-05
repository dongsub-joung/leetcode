# https://leetcode.com/problems/longest-palindromic-substring/

class Solution:
    def longestPalindrome(s: str) -> str:
        long= ""
        if len(s) <= 1:
            return s
        for i in range(len(s)):
            for j in range(len(s), i, -1):
                if len(long) >= j-i:
                    continue
                elif s[i:j] == s[i:j][::-1]:
                    long= s[i:j]

        return long

    def longestPalindrome2(self, s: str) -> str:
            if len(s) <= 1:
                return s
            i,l=0,0
            for j in range(len(s)):
                if s[j-l: j+1] == s[j-l: j+1][::-1]:
                    i, l = j-l, l+1
                    # print(s[i: i+l])
                elif j-l > 0 and s[j-l-1: j+1] == s[j-l-1: j+1][::-1]:
                    i, l = j-l-1, l+2
                    # print(s[i: i+l])
            return s[i: i+l]

result= Solution.longestPalindrome("babad")
# Output: "bab"
print(result)