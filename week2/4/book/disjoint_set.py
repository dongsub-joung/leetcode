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


disjoint_set = DisjointSet(10)
edges = [(1, 3), (2, 5), (3, 5), (4, 6), (7, 10)]
for idx, adj in edges:
    disjoint_set.union(idx, adj)
for i in range(1, 11):
    print(f"{i}의 부모: {disjoint_set.find(i)}")