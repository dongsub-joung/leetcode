use std::{cmp::Reverse, collections::binary_heap::Iter};
pub struct Solution{}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let string_valvue= i32::to_string(&x);
        let mut idx= string_valvue.len();  

        // return sizecd
        let reversed= string_valvue.as_bytes();
        for c in reversed{
            if *c == reversed[idx-1]{
                idx-=1;
                continue;
            }else{
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::is_palindrome(121);
        assert_eq!(result, true);
    }

    #[test]
    fn it_works2() {
        let result = Solution::is_palindrome(-121);
        assert_eq!(result, false);
    }

    #[test]
    fn it_works3() {
        let result = Solution::is_palindrome(10);
        assert_eq!(result, false);
    }
}
