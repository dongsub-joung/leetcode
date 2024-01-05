import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.*;

public class Main {
    public static void main(String[] args) throws IOException {
//        Input IO
        BufferedReader br= new BufferedReader(new InputStreamReader(System.in));

//        split
        StringTokenizer st= new StringTokenizer(br.readLine());

        final int N= Integer.parseInt(st.nextToken());
        final int M= Integer.parseInt(st.nextToken());

        Map<String, String> map= new HashMap<>();

        for (int i=0; i< N; i++){
            st= new StringTokenizer(br.readLine());
            var site= st.nextToken();
            var pw= st.nextToken();
            map.put(site, pw);
        }

        StringBuilder sb= new StringBuilder();
        for (int i=0; i< M; i++){
            var site= br.readLine();
            sb.append(map.get(site) + "\n");
        }

        System.out.println(sb);
    }
}