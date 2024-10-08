pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut buffer: &[u8] = "".as_bytes();
        let mut idx= 0_usize;
        let mut j= 0_usize;
        let mut pivot= 0_usize;
        for s in strs{
            if idx == 0{
                buffer= (s.clone()).as_bytes().clone();
            }

            let bytes= s.as_bytes();
            for bc in bytes{
                if buffer[j] == *bc{
                    continue;
                }else{
                    pivot= j;
                }
            }

            idx+=1; j+=1;
        }

        let mut result: &str= "";
        let mut c_buff:Vec<char>= Vec::new();
        if pivot == 0{
            return "".to_string();
        }else{
            for bc in buffer{
                c_buff.push(*bc as char);
                result= &String::from_iter(c_buff.clone());                
            }
        }

        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::longest_common_prefix(Vec::from(["flower".to_string(),"flow".to_string(),"flight".to_string()]));
        assert_eq!(result, "fl".to_string());
    }


    #[test]
    fn it_works2() {
        let result = Solution::longest_common_prefix(Vec::from( ["dog".to_string(),"racecar".to_string(),"car".to_string()]));
        assert_eq!(result, "".to_string());
    }
}


// Constraints:
// 1 <= strs.length <= 200
// 0 <= strs[i].length <= 200
// strs[i] consists of only lowercase English letters.


// miyuki@archlinux ~/g/l/w/L/init (main)> cargo test
//    Compiling init v0.1.0 (/home/miyuki/git/leetcode/week0/LongestCommonPrefix_14/init)
// error[E0716]: temporary value dropped while borrowed
//   --> src/lib.rs:11:25
//    |
// 11 |                 buffer= (s.clone()).as_bytes().clone();
//    |                         ^^^^^^^^^^^                   - temporary value is freed at the end of this statement
//    |                         |
//    |                         creates a temporary value which is freed while still in use
// ...
// 16 |                 if buffer[j] == *bc{
//    |                    --------- borrow later used here
//    |
//    = note: consider using a `let` binding to create a longer lived value

// error[E0716]: temporary value dropped while borrowed
//   --> src/lib.rs:33:26
//    |
// 33 |                 result= &String::from_iter(c_buff.clone());                
//    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^- temporary value is freed at the end of this statement
//    |                          |
//    |                          creates a temporary value which is freed while still in use
// ...
// 37 |         result.to_string()
//    |         ------ borrow later used here
//    |
//    = note: consider using a `let` binding to create a longer lived value

// For more information about this error, try `rustc --explain E0716`.
// error: could not compile `init` (lib) due to 2 previous errors
// warning: build failed, waiting for other jobs to finish...
// error: could not compile `init` (lib test) due to 2 previous errors
