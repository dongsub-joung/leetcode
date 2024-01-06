// https://leetcode.com/problems/the-k-weakest-rows-in-a-matrix/
import java.util.Arrays;
import java.util.stream.IntStream;

class Solution {
    //https://walkccc.me/LeetCode/problems/1337/#__tabbed_1_2
    public static int[] kWeakestRows(int[][] mat, int k) {
        int[] answer= new int[k];
        int[][] candidates= new int[mat.length][2];
        
        for (int i=0; i< mat.length; ++i){
            candidates[i][0]= IntStream.of(mat[i]).sum();
            candidates[i][1]= i;
        }

        Arrays.sort(candidates, (a, b) -> a[0] == b[0]
            ? a[1] - b[1]
            : a[0] - b[0]);

        for (int i =0; i<k; ++i){
            answer[i]= candidates[i][1];
        }

        return answer;
    }

    public static void main(String[] args) {
        int[][] arr = {{1, 1, 0, 0, 0},
                {1, 1, 1, 1, 0},
                {1, 0, 0, 0, 0},
                {1, 1, 0, 0, 0},
                {1, 1, 1, 1, 1}};
        int k = 3;
        int[] ans = kWeakestRows(arr, k);
//        [2,0,3]
        System.out.println(Arrays.toString(ans));
    }
}
