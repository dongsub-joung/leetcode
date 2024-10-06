struct Solution{}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::two_sum(Vec::new([2_i32,7,11,15]), 9_i32);
        assert_eq!(result, Vec::new([0,1]));
    }

    #[test]
    fn it_works() {
        let result = Solution::two_sum(Vec::new([3_i32,2,4]), 6_i32);
        assert_eq!(result, Vec::new([1,2]));
    }

    #[test]
    fn it_works() {
        let result = Solution::two_sum(Vec::new([3_i32,3]), 6_i32);
        assert_eq!(result, Vec::new([0,1]));
    }
}
