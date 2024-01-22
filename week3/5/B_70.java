// https://leetcode.com/problems/climbing-stairs/description/
// https://walkccc.me/LeetCode/problems/0070/#__tabbed_1_2

// O(n)
class Solution {
  public int climbStairs(int n) {
    // dp[i] := the number of ways to climb to the i-th stair
    int[] dp = new int[n + 1];
    dp[0] = 1;
    dp[1] = 1;

    for (int i = 2; i <= n; ++i)
      dp[i] = dp[i - 1] + dp[i - 2];

    return dp[n];
  }
}

// O(1)
class Solution {
  public int climbStairs(int n) {
    int prev1 = 1; // dp[i - 1]
    int prev2 = 1; // dp[i - 2]

    for (int i = 2; i <= n; ++i) {
      final int dp = prev1 + prev2;
      prev2 = prev1;
      prev1 = dp;
    }

    return prev1;
  }
}
