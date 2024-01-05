# https://www.acmicpc.net/problem/1966
from collections import deque
import sys

input= sys.stdin.readline

n= int(input())

for i in range(n):
    n, m = map(int, input().split())
   
    cnt= 0
    queue= deque(list(map(int, input().split())))
    while queue:
        best= max(queue)
        front= queue.popleft()
        m-= 1

        if best==front:
            cnt+=1
            if m<0:
                print(cnt)
                break
        else:
            queue.append(front)
            if m<0:
                m= len(queue)-1

