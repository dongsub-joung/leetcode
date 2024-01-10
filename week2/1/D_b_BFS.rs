use std::io::{stdin, BufRead};
use std::collections::VecDeque;

fn bfs(mut graph: Vec<Vec<i32>>, a: usize, b: usize) -> i32{
    let dx = [0, 0, 1, -1];
    let dy = [1, -1, 0, 0];

    let mut queue: VecDeque<usize> = VecDeque::new();
    
    queue.append(&mut VecDeque::from([a,b]));
    graph[a][b]= 0;
    let mut count= 1;

    while !queue.is_empty() {
        let x= queue.pop_front().unwrap();
        graph[x][y] =0;

        let mut nx;
        let mut ny;
        for i in 0..4{
            nx= x+dx[i];
            ny= y+dy[i];
        }

        if nx < 0 || nx>= graph.len() || ny < 0 || ny>= graph.len(){
            continue;
        }

        if graph[nx][ny] == 1 {
            graph[nx][ny] = 0;
            queue.append(&mut VecDeque::from([nx, ny]));
            count+= 1
        }
    }

    count
}

fn main() {
    let std= stdin();
    let mut buf= std.lock().lines();
    let input= buf.next().unwrap().unwrap();
    let n: i32= input.trim().parse().unwrap();

    let mut graph: Vec<i32>;
    let result: Vec<i32>= Vec::new();

    for _ in 0..n{
        let input2= buf.next().unwrap().unwrap();
        let input2_list: Vec<i32>= input2.trim().split_whitespace().map(|f| f.parse().unwrap()).collect();
        graph= input2_list;   
    }

    let dx = [0, 0, 1, -1];
    let dy = [1, -1, 0, 0];


}