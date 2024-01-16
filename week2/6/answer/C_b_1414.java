// https://www.acmicpc.net/problem/1414

import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;
import java.util.Scanner;

class DisjointSet {
    private int[] parent;

    public DisjointSet(int n) {
        parent = new int[n];
        for (int i = 0; i < n; i++) {
            parent[i] = i;
        }
    }

    public int find(int x) {
        if (parent[x] == x) {
            return x;
        }
        parent[x] = find(parent[x]);
        return parent[x];
    }

    public void union(int x, int y) {
        int rootX = find(x);
        int rootY = find(y);
        if (rootX == rootY) {
            return;
        }
        parent[rootX] = rootY;
    }
}

public class Charity {

    public static int kruskalCustom(int n, List<int[]> edges) {
        edges.sort(Comparator.comparingInt(edge -> edge[2]));
        DisjointSet disjointSet = new DisjointSet(n);
        int result = 0;
        int usedEdges = 0;

        for (int[] edge : edges) {
            int idx = edge[0];
            int adj = edge[1];
            int cost = edge[2];

            if (disjointSet.find(idx) != disjointSet.find(adj)) {
                disjointSet.union(idx, adj);
                result += cost;
                usedEdges++;
                if (usedEdges == n - 1) {
                    break;
                }
            }
        }

        return (usedEdges == n - 1) ? result : -1;
    }

    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        int n = scanner.nextInt();
        String[] board = new String[n];
        List<int[]> edges = new ArrayList<>();
        int total = 0;

        for (int i = 0; i < n; i++) {
            board[i] = scanner.next();
        }

        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                char t = board[i].charAt(j);
                if ('a' <= t && t <= 'z') {
                    int k = t - 'a' + 1;
                    edges.add(new int[]{i, j, k});
                    total += k;
                } else if ('A' <= t && t <= 'Z') {
                    int k = t - 'A' + 27;
                    edges.add(new int[]{i, j, k});
                    total += k;
                }
            }
        }

        int val = kruskalCustom(n, edges);
        System.out.println((val != -1) ? (total - val) : -1);
    }
}
