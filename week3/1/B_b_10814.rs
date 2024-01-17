// https://www.acmicpc.net/problem/2751

use std::io::{stdin, BufRead};

fn main() {
    let mut users: Vec<Vec<String>>= Vec::new();

    let std= stdin();
    let mut buf= std.lock().lines();

    let n: usize= buf.next().unwrap().unwrap().parse().unwrap();

    for i in 0..n{
        let mut age_and_name: Vec<String>= buf.next().unwrap().unwrap().trim().split_whitespace().map(|f| f.to_string()).collect();
        // save index
        age_and_name.push(i.to_string());
        users.push(age_and_name);
    }

    for _ in 0..n {
        let mut tokens: Vec<String>= buf.next().unwrap().unwrap().trim().split_whitespace().map(|f| f.to_string()).collect();
        let mut first= 0;
        let mut second=0;
        for (i, e) in tokens.iter().enumerate(){
            match i {
                0 => first= tokens[0].parse().unwrap(),
                1 => second= tokens[1].parse().unwrap(),
                _ => continue,
            }
        }
        arr.push([first.to_string(), second.to_string()]);
    }
    
    // if age == age
    // * -1

    

}
