// https://leetcode.com/problems/largest-number/description/
// https://walkccc.me/LeetCode/problems/0179/#__tabbed_1_2

class Solution {
  public String largestNumber(int[] nums) {
    final String s = Arrays.stream(nums)
                         .mapToObj(String::valueOf)
                         .sorted((a, b) -> (b + a).compareTo(a + b))
                         .collect(Collectors.joining(""));
    return s.startsWith("00") ? "0" : s;
  }
}
