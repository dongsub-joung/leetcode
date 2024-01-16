// https://www.acmicpc.net/problem/10026

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;

public class CountSections {
    private static int countSections(char[][] board) {
        /*
        input:
            board: 2차원 배열
        output:
            구역의 수
         */
        int n = board.length;
        boolean[][] visited = new boolean[n][n];
        int sections = 0;
        int[] dx = {0, 0, -1, 1};
        int[] dy = {-1, 1, 0, 0};

        for (int x = 0; x < n; x++) {
            for (int y = 0; y < n; y++) {
                if (!visited[x][y]) {
                    sections++;
                    char target = board[x][y];
                    visited[x][y] = true;

                    // DFS
                    int[][] stack = new int[n * n][2];
                    int top = 0;
                    stack[top][0] = x;
                    stack[top][1] = y;

                    while (top >= 0) {
                        int[] current = stack[top--];
                        int cx = current[0];
                        int cy = current[1];

                        for (int i = 0; i < 4; i++) {
                            int nx = cx + dx[i];
                            int ny = cy + dy[i];

                            if (0 <= nx && nx < n && 0 <= ny && ny < n && !visited[nx][ny] && board[nx][ny] == target) {
                                stack[++top][0] = nx;
                                stack[top][1] = ny;
                                visited[nx][ny] = true;
                            }
                        }
                    }
                }
            }
        }

        return sections;
    }

    public static void main(String[] args) throws IOException {
        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
        int n = Integer.parseInt(br.readLine());

        char[][] board = new char[n][n];


        for (int i = 0; i < n; i++) {
            String line = br.readLine().trim();
            board[i] = line.toCharArray();
        }

        int normal = countSections(board);

        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                board[i][j] = (board[i][j] == 'G') ? 'R' : board[i][j];
            }
        }

        int blind = countSections(board);

        System.out.println(normal + " " + blind);
    }
}
