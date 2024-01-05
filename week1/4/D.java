import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.PriorityQueue;

public class Main {
    public static void main(String[] args) throws IOException {
//        IO better scanner
        BufferedReader br= new BufferedReader(new InputStreamReader(System.in));

//        PriorityQueue
        PriorityQueue<Integer> queue= new PriorityQueue<>();

//        N inputed at number
        final int N= Integer.parseInt(br.readLine());

//        loop n
        for (int i=0; i< N; i++){
            int value= Integer.parseInt(br.readLine());

            if (value == 0){
//                isEmpty(): true if this collection contains no elements
                if (queue.isEmpty()) System.out.println("0");
//                poll(): the head of this queue, or null if this queue is empty
                else System.out.println(queue.poll());
            }
            else{
                queue.add(value);
            }
        }
    }
}