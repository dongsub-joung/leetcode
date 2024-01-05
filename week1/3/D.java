//https://st-lab.tistory.com/261

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.Arrays;
import java.util.StringTokenizer;


class Solution {
    static int[] arr;

    public static void main(String[] args) throws IOException {
        BufferedReader br= new BufferedReader(
                new InputStreamReader(System.in));
        final int N= Integer.parseInt(br.readLine());
        arr= new int[N];

        StringTokenizer st= new StringTokenizer(br.readLine());
        for (int i=0; i< N; i++){
            arr[i]= Integer.parseInt(st.nextToken());
        }

        Arrays.sort(arr);

        final int M= Integer.parseInt(br.readLine());
        st= new StringTokenizer(br.readLine());
        StringBuilder sb = new StringBuilder();
        for (int i=0; i<M; i++){
            if(binarySearch(Integer.parseInt(st.nextToken())) >= 0) {
                sb.append(1).append('\n');
            }
            else {
                sb.append(0).append('\n');
            }
        }
        System.out.println(sb);
    }

    private static int binarySearch(int keys) {
        int lo= 0;
        int hi= arr.length -1;
    }
}