// https://www.acmicpc.net/problem/1149

use std::{cmp::min, io::{stdin, BufRead}};

fn main() {
    const RED: usize= 0;
    const GREEN: usize= 1;
    const BLUE: usize= 2;
    let mut cost: Vec<Vec<i32>>= Vec::new();
    
    let std= stdin();
    let mut buf= std.lock().lines();
    
    let n: usize= buf.next().unwrap().unwrap().trim().parse().unwrap();
    
    for i in 0..n {
        cost.push(Vec::from([0_i32; 3]));
    }

    for i in 0..n {
        let values: Vec<i32>= buf.next().unwrap().unwrap().trim().split_whitespace().map(|f| f.parse().unwrap()).collect();
        cost[i][RED]= values[0];
        cost[i][GREEN]= values[1];
        cost[i][BLUE]= values[2];
    }

    for i in 1..n  {
        cost[i][RED]+= min(cost[i-1][GREEN], cost[i-1][BLUE]);
        cost[i][GREEN]+= min(cost[i-1][RED], cost[i-1][BLUE]);
        cost[i][BLUE]+= min(cost[i-1][RED], cost[i-1][GREEN]);
    }

    let answer= min(min(cost[n-1][RED], cost[n-1][GREEN]), cost[n-1][BLUE]);
    println!("{}", answer);
}
