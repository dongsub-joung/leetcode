# https://leetcode.com/problems/daily-temperatures/

class Solution:
    def dailyTemperatures(temperatures: list[int]) -> list[int]:
        answer= [0] * len(temperatures)
        stack= []

        for i, cur in enumerate(temperatures):
            while stack and cur > temperatures[stack[-1]]:
                print(stack[-1])
                last= stack.pop()
                answer[last]= i - last
            stack.append(i)
        return answer

# my code
def dailyTemperatures2(temperatures: list[int]) -> list[int]:
    temperatures_copy= temperatures
    results= []

    for i in range(len(temperatures)):
        point= temperatures[-1]
        for j, temper in enumerate(temperatures):
            if point < temper:
               if j-i >= 0:
                results.append(j-i)
        temperatures.remove(temperatures[-1])

    return results

# [1,1,4,2,1,1,0,0]
# print(dailyTemperatures2([73,74,75,71,69,72,76,73])) 
print(Solution.dailyTemperatures([73,74,75,71,69,72,76,73]))