# https://www.acmicpc.net/problem/1922

def prim(n, edges):
    import heapq

    graph = [[] for _ in range(n + 1)]
    for idx, adj, cost in edges:
        graph[idx].append((cost, adj))
        graph[adj].append((cost, idx))

    visited = [False] * (n + 1)
    visited[1] = True
    heap = []
    for cost, adj in graph[1]:
        heapq.heappush(heap, (cost, adj))

    result = 0
    used_edges = 0
    while used_edges < n - 1:
        cost, idx = heapq.heappop(heap)
        if visited[idx]:
            continue
        visited[idx] = True
        result += cost
        used_edges += 1
        for adj_cost, adj in graph[idx]:
            if not visited[adj]:
                heapq.heappush(heap, (adj_cost, adj))

    return result


if __name__ == '__main__':
    from sys import stdin
    input = stdin.readline
    n = int(input())
    m = int(input())
    edges = [tuple(map(int, input().split())) for _ in range(m)]
    print(prim(n, edges))