class DisjointSet:

    def __init__(self, n):
        self.parent = list(range(n + 1))

    def find(self, x):
        if self.parent[x] == x:
            return x
        self.parent[x] = self.find(self.parent[x])
        return self.parent[x]
    
    def union(self, x, y):
        root_x = self.find(x)
        root_y = self.find(y)
        if root_x == root_y:
            return
        self.parent[root_x] = root_y


def kruskal(n, edges) -> int:
    """
    n: 정점의 개수
    edges: (정점1, 정점2, 가중치)의 리스트
    """

    # 간선을 가중치 순으로 정렬
    edges.sort(key=lambda x: x[2])
    disjoint_set = DisjointSet(n)
    result = 0
    used_edges = 0

    # 가중치가 낮은 간선부터 선택
    for idx, adj, cost in edges:
        # 각 노드의 부모 노드 탐색
        # 사이클이 생기지 않는다면 간선을 선택
        # 부모 노드가 같다 = 이 간선을 선택하면 사이클이 생긴다!
        if disjoint_set.find(idx) != disjoint_set.find(adj):
            disjoint_set.union(idx, adj)
            result += cost
            used_edges += 1
            # 간선의 개수가 n - 1개가 되면 탐색 종료!
            if used_edges == n - 1:
                break

    return result