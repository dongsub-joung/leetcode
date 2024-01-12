def prim(n, edges) -> int:
    """
    n: 정점의 개수
    edges: (정점1, 정점2, 가중치)의 리스트

    최소 스패닝 트리의 가중치를 반환
    """
    import heapq

    graph = [[] for _ in range(n + 1)]
    for idx, adj, cost in edges:
        graph[idx].append((cost, adj))
        graph[adj].append((cost, idx))

    # 임의의 정점을 시작으로 선택한다.
    visited = [False] * (n + 1)
    visited[1] = True
    heap = []
    for cost, adj in graph[1]:
        heapq.heappush(heap, (cost, adj))

    result = 0
    used_edges = 0
    while used_edges < n - 1:
        # 가중치 낮은 간선을 선택한다.
        cost, idx = heapq.heappop(heap)
        # 이미 방문한 정점이라면 패스
        if visited[idx]:
            continue
        visited[idx] = True
        result += cost
        used_edges += 1
        # 선택한 정점과 연결된 간선들을 우선순위 큐에 넣는다.
        for adj_cost, adj in graph[idx]:
            if not visited[adj]:
                heapq.heappush(heap, (adj_cost, adj))

    return result