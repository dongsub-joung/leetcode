// https://leetcode.com/problems/course-schedule/description/

use std::collections::VecDeque;

struct Solution{}
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let i = prerequisites.len();
        let mut tmp = prerequisites.len();
        let size = prerequisites.len();
        let mut counter: Vec<i32>= vec![0; num_courses as usize];
        let mut que:VecDeque<i32>= VecDeque::new();

        for i in 0..size{
            let value= prerequisites[i][0];
            counter[value as usize]+=1;
        }
        for i in 0..num_courses{
            if counter[i as usize] == 0 {
                que.push_back(i as i32);
            }
        }

        let mut num= que.len();
        while !que.is_empty() {
            tmp= que.remove(que.len()).unwrap() as usize;

            for i in 0..size {
                if prerequisites[i][1] == tmp as i32 {
                    counter[prerequisites[i][0] as usize]-=1;

                    if counter[prerequisites[i][0] as usize] == 0 {
                        num+=1;  
                        que.push_back(prerequisites[i][0]);
                    }
                }
            }            
        }

        num == (num_courses as usize)
    }
}

fn main(){
    let num_courses= 2;
    let prerequisites= vec![vec![1,0],vec![0,1]];
    let reuslt= Solution::can_finish(num_courses, prerequisites);
    println!("{}", reuslt);
}