use std::{io::{stdin, BufRead}};

fn main() {
    let mut coins: Vec<i32>= Vec::new();
    let mut anwser= 0;
    
    let std= stdin();
    let mut buf= std.lock().lines();
    let n_and_k: Vec<usize>= buf.next().unwrap().unwrap()
        .trim().split_whitespace()
        .map(|f| f.parse().unwrap()).collect();
    let n= n_and_k[0];
    let k= move || n_and_k[1];

    for i in 0..n {
        let coin: i32= buf.next().unwrap().unwrap().parse().unwrap();
        coins.push(coin);
    }

    let mut k= k();
    for i in (0..n).rev() {
        anwser += k/ coins[i] as usize;
        k %= coins[i] as usize;
    }

    println!("{}", anwser);
}
