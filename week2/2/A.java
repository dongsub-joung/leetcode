// https://leetcode.com/problems/subsets/
// Source code: https://github.com/andavid/leetcode-java/blob/master/en/78.subsets.java

import java.io.*;
import java.util.ArrayList;
import java.util.List;

public class Main {
    public List<List<Integer>> subsets(int[] nums){
        List<List<Integer>> answer = new ArrayList<>();

        backtrace(nums, 0, new ArrayList<Integer>(), answer);

        return answer;
    }

    private void backtrace(int[] nums, int start, ArrayList<Integer> track, List<List<Integer>> answer) {
        answer.add(new ArrayList<>(track));

        for (int i= start; i< nums.length; i++){
            track.add(nums[i]);
            backtrace(nums, i+1, track, answer);

            track.remove(track.size() - 1);
        }
    }

    public static void main(String[] args) throws IOException {
        int[] input= new int[]{1,2,3};
//        [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
   	Main main= new Main();
        var result= main.subsets(input);
        for (var list : result){
            for (var e : list){
                System.out.print(e);
            }
            System.out.println();
        }
    }
}
