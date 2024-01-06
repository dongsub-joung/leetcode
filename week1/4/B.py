# https://leetcode.com/problems/the-k-weakest-rows-in-a-matrix/
import heapq
import collections

 

# score = soldiersCount * rows + currentRowIndex
class Solution:
    # 1. sum - sorted
    def kWeakestRows(mat: list[list[int]], k: int) -> list[int]:
        answers_dict= {}
        for i in range(len(mat)):
            # save dictionaly, index i(key): sumed(value)
            answers_dict[i]= sum(mat[i])
        
        # <built-in method get of dict object at 0x7f6f7223e740>
        print(answers_dict.get)
        # question
        answers= sorted(answers_dict, key=answers_dict.get)

        return answers[:k]

    # 2. Counter
    # https://geonoo.tistory.com/112
    def kWeakestRows2(mat: list[list[int]], k: int) -> list[int]:
        answer= []
        s= collections.defaultdict(int)

        for i in range(len(mat)):
            c= collections.Counter(mat[i])
        # 인덱스를 키로 1의 개수 만큼으로 하는 딕셔너리
            s[i]= c[1]
        
        s= sorted(s.items(), key=lambda x : x[1])
        for key, value in s:
            answer.append(key)

        return answer[:k]


# heap
# https://maxming0.github.io/2021/02/15/The-K-Weakest-Rows-in-a-Matrix/
    def kWeakestRows_heap(mat: list[list[int]], k: int) -> list[int]:
        power= []
        
        for r, row in enumerate(mat):
            temp= 0
            for n in row:
                if n:
                    temp+= 1
                else:
                    break
            power.append((temp, r))

        # heapify(): from list to heap
        heapq.heapify(power)

        return [heapq.heappop(power)[1] for _ in range(k)]
            


mat= [[1,1,0,0,0],
 [1,1,1,1,0],
 [1,0,0,0,0],
 [1,1,0,0,0],
 [1,1,1,1,1]]
k= 3
# Output: [2,0,3]
result= Solution.kWeakestRows(mat, k)
print(result)
result2= Solution.kWeakestRows2(mat, k)
print(result2)
result3= Solution.kWeakestRows_heap(mat, k)
print(result3)