# https://www.acmicpc.net/problem/1931
from sys import stdin

input = stdin.readline

n = int(input())
meetings = [tuple(map(int, input().split())) for _ in range(n)]
meetings.sort(key=lambda x: (x[1], x[0]))
ans = 0
now = 0
for start, end in meetings:
    if now <= start:
        now = end
        ans += 1
print(ans)
