# https://leetcode.com/problems/the-k-weakest-rows-in-a-matrix/

class Solution:
    def kWeakestRows(mat: list[list[int]], k: int) -> list[int]:
        answers= {}
        for i in range(len(mat)):
            # save dictionaly, index i(key): sumed(value)
            answers[i]= sum(mat[i])
        
        # 
        answers= sorted(answers, key=answers.get)

        return answers[:k]

mat= [[1,1,0,0,0],
 [1,1,1,1,0],
 [1,0,0,0,0],
 [1,1,0,0,0],
 [1,1,1,1,1]]
k= 3
# Output: [2,0,3]
result= Solution.kWeakestRows(mat, k)
print(result)
