// 백준 5397 키로거 rust
// https://www.acmicpc.net/problem/5397

use std::io::{stdin, BufRead};
fn main() {
    

    let std= stdin();
    let mut buf= std.lock().lines();
    let input= buf.next().unwrap().unwrap();

    let t: i32= input.trim().parse().unwrap();
    let mut inputes: Vec<char>;

    for _ in 0..t{
        let mut left: Vec<char> = Vec::new();
        let mut right: Vec<char> = Vec::new();

        let input2= buf.next().unwrap().unwrap();
        inputes= input2.trim().chars().collect();
        
        for c in inputes{
            match c {
                '<' => {
                    if !left.is_empty(){
                        right.push(left.pop().unwrap());
                    }
                },

                '>' => {
                    if !right.is_empty() {
                        left.push(right.pop().unwrap());
                    }
                },

                '-' => {
                    if !left.is_empty() {
                        left.pop();
                    }
                },

                _ => left.push(c),
            }
        }

        while !left.is_empty() {
            right.push(left.pop().unwrap());
        }

        // let results: Vec<String>= Vec::new();
        let mut answer= String::new();
        while !right.is_empty() {
            answer.push(right.pop().unwrap());  
        }
        println!("{}", answer);
        
        answer= String::new();
        
// 2
// <<BP<A>>Cd-
// ThIsIsS3Cr3t
        
    }
}
