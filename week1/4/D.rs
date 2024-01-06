// 시간 초과
use std::{io::{stdin, BufReader, BufRead}, collections::BinaryHeap};

fn main() {
    let std= stdin();
    let mut buf= std.lock().lines();
    let input= buf.next().unwrap().unwrap();
    let mut input_str= input.trim().split_whitespace();

    let n: i32= input_str.next().unwrap().parse().unwrap();
    
    let mut heap: BinaryHeap<i32>= BinaryHeap::new();
    for _ in 0..n{
        let inner_input= buf.next().unwrap().unwrap();
        let inner_input_number: i32= inner_input.trim().parse().unwrap();

        if inner_input_number == 0{
            if heap.len() > 0{
                println!("{}", heap.pop().unwrap())
            }else{
                println!("0");
            }
        }else{
            heap.push(inner_input_number);
        }
    }
}
