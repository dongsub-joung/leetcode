// https://www.acmicpc.net/problem/15903
use std::{io::{stdin, BufRead}, collections::{BinaryHeap, binary_heap}};
use std::cmp;

fn main() {
    let std= stdin();
    let mut buf= std.lock().lines();
    
    let input= buf.next().unwrap().unwrap();
    let nms: Vec<i32>= input.trim().split_whitespace().map(|f| f.parse().unwrap()).collect();
    
    let input2= buf.next().unwrap().unwrap();
    let As: Vec<i32>= input2.trim().split_whitespace().map(|f| f.parse().unwrap()).collect();

    let n= nms[0];
    let m= nms[1];
    
    let mut heap: BinaryHeap<i32>= BinaryHeap::new();
    for a in As{
        heap.push(a * -1);
    }

    for _ in 0..m{
        let mut mini= i32::MAX;
        let mut sec_mini= 0;
        mini= heap.pop().unwrap();
        sec_mini= heap.pop().unwrap();
        let s: i32=  (mini+sec_mini) * -1;
        heap.push(s * -1);
        heap.push(s * -1);
    }

    let answer: i32= heap.iter().map(|f| f * -1).sum();

    println!("{}", answer);
}


// mini heap
use std::{io::{stdin, BufRead}, collections::{BinaryHeap, binary_heap}};
use std::cmp;

fn main() {
    let std= stdin();
    let mut buf= std.lock().lines();
    
    let input= buf.next().unwrap().unwrap();
    let nms: Vec<u32>= input.trim().split_whitespace().map(|f| f.parse().unwrap()).collect();
    
    let input2= buf.next().unwrap().unwrap();
    let As: Vec<u32>= input2.trim().split_whitespace().map(|f| f.parse().unwrap()).collect();

    let n= nms[0];
    let m= nms[1];
    
    let mut heap: BinaryHeap<u32>= BinaryHeap::new();
    for a in As{
        heap.push(a);
    }

    for _ in 0..m{
        let mut mini= u32::MAX;
        let mut sec_mini= 0;

        {
            let mut mini_local= heap.pop().unwrap();
            let mut stack: Vec<u32>= Vec::new();
            
            for _ in 1..heap.len(){
                mini_local= cmp::min(mini, heap.pop().unwrap());
                stack.push(heap.pop().unwrap());
            }
            stack.sort();
            mini= mini_local;
            sec_mini= stack[0];
        }

        println!("{}: {}", mini, sec_mini);
        let s: u32=  mini+sec_mini;
        heap.push(s);
        heap.push(s);
    }

    for e in &heap {
        print!("{} ", e);
    }
    let answer: u32= heap.iter().sum();

    println!("{}", answer);
}

// Max heap
use std::cmp::Reverse;
use std::{io::{stdin, BufRead}, collections::BinaryHeap};
fn main() {
    let std= stdin();
    let mut buf= std.lock().lines();
    
    let input= buf.next().unwrap().unwrap();
    let nms: Vec<u32>= input.trim().split_whitespace().map(|f| f.parse().unwrap()).collect();
    
    let input2= buf.next().unwrap().unwrap();
    let mut As: Vec<u32>= input2.trim().split_whitespace().map(|f| f.parse().unwrap()).collect();

    let n= nms[0];
    let m= nms[1];
    
    let mut heap: BinaryHeap<Reverse<u32>>= BinaryHeap::new();
    

    for a in As{
        heap.push(Reverse(a));
    }

    for _ in 0..m{    
        let s: u32= heap.pop().unwrap().0 + heap.pop().unwrap().0;
        heap.push(Reverse(s));
        heap.push(Reverse(s));
    }

    let answer: u32= heap.iter().map(|f| f.0).sum();
    println!("{}", answer);
}


// My code
// test case 1,2은 통과
// 1: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=ff4d6eae5cb4b5ab1f882a5547e6f49e
// 2: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=7c7f2ae91437c70827836b8ba3897d6f
use std::io::{stdin, BufRead};
fn main() {
    let std= stdin();
    let mut buf= std.lock().lines();
    
    let input= buf.next().unwrap().unwrap();
    let nms: Vec<u32>= input.trim().split_whitespace().map(|f| f.parse().unwrap()).collect();
    
    let input2= buf.next().unwrap().unwrap();
    let mut As: Vec<u32>= input2.trim().split_whitespace().map(|f| f.parse().unwrap()).collect();

    let n= nms[0];
    let m= nms[1];
    
    for _ in 0..m{    
        As.sort();
        let mut frist_min= As[0];
        let mut second_min= As[1];
        let sumed= frist_min+second_min;
        As[0]= sumed.clone();
        As[1]= sumed.clone();
    }

    let answer= As.iter().sum::<u32>();
    println!("{}", answer);
}
