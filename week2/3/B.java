// https://www.acmicpc.net/problem/11725
// https://velog.io/@darak/BJ-11725-%ED%8A%B8%EB%A6%AC%EC%9D%98-%EB%B6%80%EB%AA%A8-%EC%B0%BE%EA%B8%B0-Java

import java.util.ArrayList;
import java.util.LinkedList;
import java.util.Queue;
import java.util.Scanner;

public class BJ_11725 {

	public static void main(String[] args) {
		Scanner sc = new Scanner(System.in);
		int n = sc.nextInt();

		ArrayList<ArrayList<Integer>> tree = new ArrayList<>();
		for (int i = 0; i < n; i++)
			tree.add(new ArrayList<>());

		for (int i = 0; i < n - 1; i++) {
			int n1 = sc.nextInt() - 1;
			int n2 = sc.nextInt() - 1;
			tree.get(n1).add(n2);
			tree.get(n2).add(n1);
		}

		boolean[] visited = new boolean[n]; 
		int[] parentNode = new int[n];

		// BFS
		Queue<Integer> queue = new LinkedList<>();
		queue.add(0);
		visited[0] = true;
		while (!queue.isEmpty()) {
			int v = queue.remove();
			for (int node : tree.get(v))
				if (!visited[node]) {
					visited[node] = true;
					queue.add(node);
					parentNode[node] = v;
				}
		}

		for (int i = 1; i < n; i++)
			System.out.println(parentNode[i] + 1);
	}

}
