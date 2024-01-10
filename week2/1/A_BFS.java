// https://www.acmicpc.net/problem/5397
// BFS
import java.util.*;

public class Main {
    public static final String[] TABLE = new String[]{"", "", "abc", "def", "ghi", "jkl", "mno",
            "pqrs", "tuv", "wxyz"};
    public List<String> litterCombinations(String digits) {
        if(digits.isEmpty()){
            return Collections.emptyList();
        }

        char[] digit_chars= digits.toCharArray();
        int[] numbers= new int[digit_chars.length];
        for (int i =0; i< digit_chars.length; i++){
            numbers[i]= digit_chars[i] - '0';
        }

        int length= numbers.length;

        return litterCombinationUtil(numbers, length);
    }

    private List<String> litterCombinationUtil(int[] numbers, int length) {
        List<String> list= new ArrayList<>();
        Queue<String> queue= new LinkedList<>();
        queue.add("");

        String letterCombination, letters;
        while (!queue.isEmpty()){
            letterCombination= queue.poll();

            if (letterCombination.length() == length){
                list.add(letterCombination);
                continue;
            }
            var _size= letterCombination.length();
            letters= TABLE[numbers[_size]];
            for (int i = 0; i< letters.length(); i++){
                queue.add(letterCombination + letters.charAt(i));
            }
        }

        return list;
    }
}