// https://www.acmicpc.net/problem/5397
// https://walkccc.me/LeetCode/problems/0046/?h=46.+permutations#__tabbed_1_2
import java.util.*;

public class Main {

    final static String[] digitToLetters = {"",    "",    "abc",  "def", "ghi",
            "jkl", "mno", "pqrs", "tuv", "wxyz"};
    public List<String> letterCombinations(String digits){
        if (digits.isEmpty())
            return new ArrayList<>();

        List<String> answer= new ArrayList<>();

        dfs(digits, 0, new StringBuilder(), answer);

        return answer;
    }

    private void dfs(String digits, int i, StringBuilder sb, List<String> answer) {
        if (i == digits.length()) {
            answer.add(sb.toString());
            return;
        }
        var _size= digits.charAt(i) - '0';
        for (var c : digitToLetters[_size].toCharArray()){
            sb.append(c);
            dfs(digits, i+1, sb, answer);
            sb.deleteCharAt(sb.length()-1);
        }
    }
}