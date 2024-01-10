// https://leetcode.com/problems/permutations/
// https://walkccc.me/LeetCode/problems/0046/?h=46.+permutations#__tabbed_1_2
import java.util.*;

public class Main {
    public List<List<Integer>> permute(int[] nums){
        List<List<Integer>> answer= new ArrayList<>();

        dfs(nums, new boolean[nums.length], new ArrayList<>(), answer);

        return answer;
    }

    private void dfs(int[] nums, boolean[] used, ArrayList<Integer> path, List<List<Integer>> answer) {
        if (path.size() == nums.length){
            answer.add(new ArrayList<>(path));
            return;
        }

        for (int i =0; i< nums.length; ++i){
            if (used[i]) continue;
            used[i]= true;
            path.add(nums[i]);
            dfs(nums,used,path,answer);
            path.remove(path.size() - 1);
            used[i]= false;
        }
    }
}