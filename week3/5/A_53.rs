// https://leetcode.com/problems/maximum-subarray/description/

use std::cmp::max;

struct Solution{}
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut answer= i32::MIN;
        let mut sum= 0;

        for num in nums  {
            sum= max(num, sum+num);
            answer= max(answer, sum);
        }

        answer
    }
}
fn main() {
    let nums= Vec::from([-2,1,-3,4,-1,2,1,-5,4]);
    let result= Solution::max_sub_array(nums);
    println!("{}", result);
}
