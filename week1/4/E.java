import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.Collections;
import java.util.PriorityQueue;

public class Main {
    public static void main(String[] args) throws IOException {
//        Input IO
        BufferedReader br= new BufferedReader(new InputStreamReader(System.in));

//        PriorityQueue 내림차순 정렬
        PriorityQueue<Integer> queue= new PriorityQueue<>(Collections.reverseOrder());

        final int N= Integer.parseInt(br.readLine());

//       StringBuilder : https://onlyfor-me-blog.tistory.com/317
//        불변 string -> 동적 String buffer
        StringBuilder sb = new StringBuilder();

        for (int i =0; i<N; i++){
            int num= Integer.parseInt(br.readLine());

            if(num == 0){
//                삼항연산자
//                poll: the head of this queue, or null if this queue is empty
//                개행 \n
                sb.append(queue.size() == 0 ? 0 : queue.poll()).append('\n');
            }else{
                queue.add(num);
            }

            System.out.println(
                    sb.toString()
            );
        }
    }
}