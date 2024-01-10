# https://www.acmicpc.net/problem/2606
# code ref: https://chanos.tistory.com/entry/%EB%B0%B1%EC%A4%80-2606%EB%B2%88-%EB%B0%94%EC%9D%B4%EB%9F%AC%EC%8A%A4-%ED%8C%8C%EC%9D%B4%EC%8D%AC-%EB%AC%B8%EC%A0%9C-%ED%92%80%EC%9D%B4-DFS%EC%99%80-BFS-%EC%B0%A8%EC%9D%B4#DFS%---Depth%--First%--Search%-C%--%EA%B-%-A%EC%-D%B-%--%EC%-A%B-%EC%--%A-%--%ED%--%--%EC%--%---%C-%A-


import sys
input= sys.stdin.readline

cnt=0
def BFS(virus):
    global cnt
    visited[virus] = 1
    queue = [virus]
    while queue:
        for i in network[queue.pop()]:
            if (visited[i]==0):
                visited[i]=1
                queue.insert(0, i)
                cnt+=1

N= int(input())
link = int(input())

network = [[]*(N+1) for _ in range(N+1)]

for i in range(link):
    a, b = map(int, input().split())
    network[a].append(b)
    network[b].append(a)

visited = [0]*(N+1)
BFS(1)
#DFS(1)
print(cnt)