import java.util.Set;
import java.util.stream.Collectors;

class Solution {
    public int numJewelsInStones(String jewels, String stones) {
        int answer= 0;

        Set<Character> jewelsSet= jewels.chars().mapToObj(
                c -> (char) c
        ).collect(Collectors.toSet());

        for(char stone: stones.toCharArray()){
            if (jewelsSet.contains(stone)) ++answer;
        }

        return answer;
    }

    public static void main(String[] args) {
        Solution solution=new Solution();
        var result= solution.numJewelsInStones("aA", "aAAbbbb");
        System.out.println(result);
    }
}
