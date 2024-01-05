# https://leetcode.com/problems/group-anagrams/

class Solution:
    def groupAnagrams(strs: list[str]) -> list[list[str]]:
        # allocate dictionary 
        anagrams= {}

        # loop strs list
        for i in range(len(strs)):
            #  ex) sorted() return value ['a', 'a', 'd', 'g', 't', 'y']
            #  -> use join -> str type
            _str= ''.join(sorted(strs[i]))

            if _str not in anagrams:
                # add key: value in dictionary
                anagrams[_str]= [strs[i]]
            else:
                # append value in key "_str"
                anagrams[_str].append(strs[i])
        # from dictionary to list 
        return list(anagrams.values())


# 처음엔 아스키 코드의 합이 동일한 것을 이용해서 풀려고 시도했지만 인덱스 반환에서 실패
    def groupAnagrams2(strs: list[str]) -> list[list[str]]:
        _sum=0 
        sum_list= []
        for _str in strs:
            for c in _str:
                _sum+=ord(c)
            sum_list.append(_sum)
            _sum= 0

        print(sum_list)


result= Solution.groupAnagrams(["eat","tea","tan","ate","nat","bat"])
# result= Solution.groupAnagrams2(["eat","tea","tan","ate","nat","bat"])
print(result)