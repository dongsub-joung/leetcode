// https://www.acmicpc.net/problem/7562

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.LinkedList;
import java.util.Queue;

public class ChessMoves {

    static int[] dx = {2, 2, 1, 1, -1, -1, -2, -2};
    static int[] dy = {1, -1, 2, -2, 2, -2, 1, -1};

    public static void main(String[] args) throws IOException {
        /*
        input:
            start: 시작점 (x, y)
            end: 도착점 (x, y)
            size: 체스판 크기 n
        output:
            최소 이동 횟수
         */
        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
        int t = Integer.parseInt(br.readLine());

        for (int i = 0; i < t; i++) {
            int n = Integer.parseInt(br.readLine());
            String[] input = br.readLine().split(" ");
            int x = Integer.parseInt(input[0]);
            int y = Integer.parseInt(input[1]);
            input = br.readLine().split(" ");
            int fx = Integer.parseInt(input[0]);
            int fy = Integer.parseInt(input[1]);

            System.out.println(bfs(new int[]{x, y}, new int[]{fx, fy}, n));
        }
    }

    private static int bfs(int[] start, int[] end, int size) {
        Queue<int[]> queue = new LinkedList<>();
        int[][] board = new int[size][size];
        for (int i = 0; i < size; i++) {
            for (int j = 0; j < size; j++) {
                board[i][j] = -1;
            }
        }

        queue.offer(start);
        board[start[0]][start[1]] = 0;

        while (!queue.isEmpty()) {
            int[] current = queue.poll();
            int x = current[0];
            int y = current[1];

            if (x == end[0] && y == end[1]) {
                return board[x][y];
            }

            for (int i = 0; i < 8; i++) {
                int nx = x + dx[i];
                int ny = y + dy[i];

                if (0 <= nx && nx < size && 0 <= ny && ny < size && board[nx][ny] == -1) {
                    board[nx][ny] = board[x][y] + 1;
                    queue.offer(new int[]{nx, ny});
                }
            }
        }

        return -1; // 경로가 없을 때
    }
}