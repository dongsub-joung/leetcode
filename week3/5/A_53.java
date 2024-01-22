// https://leetcode.com/problems/maximum-subarray/description/
// https://walkccc.me/LeetCode/problems/0053/

// O(n)
class Solution {
  public int maxSubArray(int[] nums) {
    // dp[i] := the maximum sum subarray ending in i
    int[] dp = new int[nums.length];

    dp[0] = nums[0];
    for (int i = 1; i < nums.length; ++i)
      dp[i] = Math.max(nums[i], dp[i - 1] + nums[i]);

    return Arrays.stream(dp).max().getAsInt();
  }
}

// O(1)
class Solution {
  public int maxSubArray(int[] nums) {
    int ans = Integer.MIN_VALUE;
    int sum = 0;

    for (final int num : nums) {
      sum = Math.max(num, sum + num);
      ans = Math.max(ans, sum);
    }

    return ans;
  }
}
