//https://www.acmicpc.net/problem/20055
//https://velog.io/@bookae/BOJ-20055.-%EC%BB%A8%EB%B2%A0%EC%9D%B4%EC%96%B4-%EB%B2%A8%ED%8A%B8-%EC%9C%84%EC%9D%98-%EB%A1%9C%EB%B4%87-Java

import java.io.*;
import java.util.StringTokenizer;

public class Main{
    public static int n, k;
    public static int[] a_list;
    public static boolean[] belt;

    // 내구도가 0인 칸 세기 - loop end condition
    static boolean check() {
        int cnt=0;
        for(int a: a_list) {
            if(a==0) cnt++;
            if(cnt>=k) return false;
        }

        return true;
    }
    public static int init() {
        int step= 0;
        belt= new boolean[n];

        while (check()){
            step++;

//            copy(pre up)
            //1번: 벨트 회전
            int last_value_idx= 2*n-1;
            int temp = a_list[last_value_idx];
            for(int i=last_value_idx; i>0; i--)
                a_list[i] = a_list[i-1];
            a_list[0] = temp;

//            copy(pre down)
            int down_last_Val_idx= n-1;
            for(int i=down_last_Val_idx; i>0; i--) {
                belt[i] = belt[i-1];
            }

            belt[0] = false;
            belt[n-1] = false;

            //2번: 로봇 이동
            for(int i=down_last_Val_idx;i>0;i--) {
                if( (!belt[i-1]) || (belt[i]) || (a_list[i]<1) )
                    continue;

                a_list[i]--;

                belt[i] = true;
                belt[i-1] = false;
            }

            //3번: 올리기
            if(a_list[0]<=0)
                continue;
            belt[0] = true;
            a_list[0]--;
        }

        return step;
    }

    public static void main(String[] args) throws IOException {
        BufferedReader br= new BufferedReader(new InputStreamReader(System.in));
        BufferedWriter bw= new BufferedWriter(new OutputStreamWriter(System.out));

//        n= 3;
//        k= 2;
        StringTokenizer st= new StringTokenizer(br.readLine());
        n = Integer.parseInt(st.nextToken());
        k = Integer.parseInt(st.nextToken());

//        a_list= new int[]{1, 2, 1, 2, 1, 2};
        final int _size= 2*n;
        st= new StringTokenizer(br.readLine());
        a_list= new int[_size];
        for (int i=0; i<_size; i++){
            a_list[i]= Integer.parseInt(st.nextToken());
        }

        int result= init();
//        2
        System.out.println(result);
    }
}
