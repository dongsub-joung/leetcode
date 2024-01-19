# https://www.acmicpc.net/problem/13975

from sys import stdin
from heapq import heapify, heappop, heappush

input = stdin.readline

for _ in range(int(input())):
    n = int(input())
    files = list(map(int, input().split()))
    heapify(files)
    ans = 0
    for _ in range(n - 1):
        a = heappop(files)
        b = heappop(files)
        ans += a + b
        heappush(files, a + b)
    print(ans)
