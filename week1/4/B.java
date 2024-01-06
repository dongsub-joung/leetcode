//https://st-lab.tistory.com/261

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.Arrays;
import java.util.StringTokenizer;

//https://tutorialcup.com/leetcode-solutions/the-k-weakest-rows-in-a-matrix-leetcode-solution.htm
class Solution {
    public static int numOnes(int[] row){

    }

    public static int[] kWeakestRows(int[][] mat, int k) {

    }
    public static void main(String[] args) {
        int [][] arr ={      {1,1,0,0,0 },
                { 1,1,1,1,0 },
                { 1,0,0,0,0 },
                { 1,1,0,0,0 },
                { 1,1,1,1,1 }};
        int k=3;
        int[]ans=kWeakestRows(arr,k);
        System.out.println(Arrays.toString(ans));
    }
}