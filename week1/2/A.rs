// https://github.com/doocs/leetcode/blob/main/solution/0200-0299/0225.Implement%20Stack%20using%20Queues/README_EN.md

use std::collections::VecDeque;

struct MyStack{
    q_1: VecDeque<i32>,
    q_2: VecDeque<i32>,
    index: i32,
}

impl MyStack {
    fn new() -> Self {
        Self{
            q_1: VecDeque::new()
            q_2: VecDeque::new()
            index: 1_i32,
        }
    }

    fn move_data(&mut self){
        while !self.q_1.is_empty(){
            self.q_2.push_back(self.pop_front().unwrap());
        }
        let tmp= self.q_1.clone();
        self.q_1= self.q_2.clone();
        self.q_2= tmp;
    }
    
    fn push(&mut self, x: i32) {
        self.q_2.push_back(x);
        self.move_data();
    }

    fn pop(&mut self) -> i32 {
        self.q_1.pop_front().unwrap()
    }

    fn top(&mut self) -> i32 {
        *self.q_1.front().unwrap()
    }

    fn empty(&self) -> bool {
        self.q_1.is_empty()
    }
}