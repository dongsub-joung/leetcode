# https://www.acmicpc.net/problem/1920
import sys
from collections import Counter

input= sys.stdin.readline

n= int(input())
# n= 5
As= list(map(int, input().split()))
# As= [4, 1, 5, 2, 3]

m= int(input())
# m=5
Ams= list(map(int, input().split()))
# Ams= [1, 3, 7, 9, 5]

results= []
counter_dict= Counter(As)
for Am in Ams:
    if counter_dict[Am]:
        results.append(1)
    else:
        results.append(0)

for result in results:
    print(result)