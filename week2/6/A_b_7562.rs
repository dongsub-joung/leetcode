// https://www.acmicpc.net/problem/7562

use std::io::{stdin, BufReader, BufRead};

pub struct Point{
    x: i32,
    y: i32,
    cnt: i32,
}
impl Point {
    pub fn new(x:i32, y:i32)->Self{ 
        Self { x, y, cnt : 0 }
    }

    pub fn new_wiht_cnt(x:i32, y:i32, cnt:i32)->Self{ 
        Self { x, y, cnt }
    }
}

fn main() {
    let range_x= [-1, -2, -2, -1, 1, 2, 2, 1];
    let range_y= [-2, -1, 1, 2, 2, 1, -1, -2];
    let mut results: Vec<usize>= Vec::new();

    let std= stdin();
    let mut buf= std.lock().lines();
    let input= buf.next().unwrap().unwrap();
    let input_n: usize= input.trim().parse().unwrap();

    let mut arr:[i32; 0];
    for _ in 0..input_n{
        let max_len_one_side: usize= buf.next().unwrap().unwrap().parse().unwrap();
        
        let current_koma_xy: Vec<i32>= buf.next().unwrap().unwrap()
            .trim().split_whitespace().map(|f| f.parse().unwrap()).collect();
        let will_koma_xy: Vec<i32>= buf.next().unwrap().unwrap()
            .trim().split_whitespace().map(|f| f.parse().unwrap()).collect();

        const X: usize= 0;
        const Y: usize= 0;
        if &current_koma_xy[X] == &will_koma_xy[X] 
            && &current_koma_xy[Y] == &will_koma_xy[Y]{
            results.push(0_usize);
        }

        // progress

    }
}