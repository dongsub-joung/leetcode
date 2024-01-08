// https://www.acmicpc.net/problem/5397
import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.*;

public class D {
    public static void main(String[] args) throws IOException {
        BufferedReader br= new BufferedReader(new InputStreamReader(System.in));
        StringBuilder sb= new StringBuilder();

        final int T= Integer.parseInt(br.readLine());
        String[] inputs;

        for (int i=0; i<T; i++){
            Stack<String> left= new Stack<>();
            Stack<String> right= new Stack<>();

            // to chars array
            inputs = br.readLine().split("");

            for (var str : inputs) {
                switch (str) {
                    case "<":
                    // if left values are..
                        if(!left.isEmpty()){
                            // transfer from left to right 
                            right.push(left.pop());
                        }
                        break;

                    case ">":
                    //  right values are..
                        if (!right.isEmpty()) {
                            // transfer from right to left                             
                            left.push(right.pop());
                        }
                        break;
                        
                    case "-":
                        if (!left.isEmpty()) {
                            left.pop();
                        }
                        break;
                    
                    default:
                        left.push(str);
                }
            }

            
            // if left values remain..
            while (!left.isEmpty()) {
                right.push(left.pop());
            }

            // consume right value using pop
            while (!right.isEmpty()) {
                // new Stringbuilder
                sb.append(right.pop());
            }
            sb.append("\n");
        }
        
        System.out.println(sb.toString());
    }
}