// https://www.acmicpc.net/problem/7562
// https://www.acmicpc.net/problem/7562

use std::borrow::Borrow;
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

pub fn bfs(arr: Vec<i32>, mut points: Vec<Point>, max_len_one_side: usize) -> i32{

    const RANGE_X:[i32; 8]= [-1, -2, -2, -1, 1, 2, 2, 1];
    const RANGE_Y:[i32; 8]=[-2, -1, 1, 2, 2, 1, -1, -2];

    let mut deque: VecDeque<Point> = VecDeque::new();
    let first_point= points[0].try_into().unwrap();
    deque.push_back(first_point);

    let mut visited: Vec<Vec<bool>>= Vec::new();
    visited[points[0].x as usize][points[0].y as usize]= true;

    while !deque.is_empty() {
        let point= deque.pop_front().unwrap();

        if &point.x == &points[1].x 
            && &point.y == &points[1].y{
                return point.cnt;
        }

        for i in 0..8 {
            let dx= point.x + RANGE_X[i];
            let dy= point.y + RANGE_Y[i];

            if dx < 0 || dy< 0 
                || dx >= max_len_one_side as i32 || dy>= max_len_one_side as i32{
                continue;
            }

            if !visited[dx as usize][dy as usize] {
                visited[dx as usize][dy as usize]= true;
                let point_with_cnt= Point::new_wiht_cnt(dx, dy, point.cnt + 1);
                deque.push_back(point_with_cnt);
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
