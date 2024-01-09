// https://www.acmicpc.net/problem/15903
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
