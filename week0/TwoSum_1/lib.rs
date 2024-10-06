struct Solution{}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i= 1;
        let mut pivots= Vec::new();
        
        for n in &nums{
            let nums_c= nums.clone();
            if n+nums_c[i] == target{
                pivots.push((i-1) as i32);
                pivots.push(i as i32);
                
                return pivots;
            }
            i+=1;
        }
        pivots
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::two_sum(Vec::from([2_i32,7,11,15]), 9_i32);
        assert_eq!(result, Vec::from([0_i32,1]));
    }

    #[test]
    fn it_works2() {
        let result = Solution::two_sum(Vec::from([3_i32,2,4]), 6_i32);
        assert_eq!(result, Vec::from([1_i32,2]));
    }

    #[test]
    fn it_works3() {
        let result = Solution::two_sum(Vec::from([3_i32,3]), 6_i32);
        assert_eq!(result, Vec::from([0_i32,1]));
    }
}
