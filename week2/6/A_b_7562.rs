// https://www.acmicpc.net/problem/7562
// 재귀가 있어 러스트로 못푸는 문제, 소유권 에러가 난다.

use std::io::{stdin, BufReader, BufRead};
use std::collections::VecDeque;

struct Point{
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

pub fn bfs(arr: Vec<i32>, points: Vec<Point>, max_len_one_side: usize) -> i32{

    let mut deque: VecDeque<&Point> = VecDeque::new();
    deque.push_back(&points[0]);

    let mut visited: Vec<Vec<bool>>= Vec::new();
    visited[points[0].x as usize][points[0].y as usize]= true;

    while !deque.is_empty() {
        let point= deque.pop_front().unwrap();

        if &point.x == &points[1].x 
            && &point.y == &points[1].y{
                return point.cnt;
        }

        for i in 0..8 {
            let dx= point.x + range_x[i];
            let dy= point.y + range_y[i];

            if dx < 0 || dy< 0 
                || dx >= max_len_one_side as i32 || dy>= max_len_one_side as i32{
                continue;
            }

            if !visited[dx as usize][dy as usize] {
                visited[dx as usize][dy as usize]= true;
                deque.push_back(&Point::new_wiht_cnt(dx, dy, point.cnt + 1));
            }
        }

    }
    return -1;
}

fn main() {

    let mut results: Vec<i32>= Vec::new();
    let mut two_dimention_arrs: Vec<Vec<i32>>= Vec::new();
    let mut points: Vec<Point>= Vec::new();
    let mut arr: Vec<i32>= Vec::new();

    let std= stdin();
    let mut buf= std.lock().lines();
    let input= buf.next().unwrap().unwrap();
    let input_n: usize= input.trim().parse().unwrap();

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
            results.push(0_i32);
        }

        // value movedd
        points.push(Point::new(current_koma_xy[X], current_koma_xy[Y]));
        points.push(Point::new(will_koma_xy[X], will_koma_xy[Y]));

        // progress
        results.push(bfs(arr.clone(), points, max_len_one_side,));
    }
}