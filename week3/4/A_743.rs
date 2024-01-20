use std::collections::HashMap;

struct Solution{}
impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        // let mut graph:Vec<(i32, i32)>= Vec::new();
        let mut graph:HashMap<i32, i32>= HashMap::new();
        
        for time in times{
            let u= time[0] - 1;
            let v= time[1] -1;
            let w= time[2];

            graph[u as usize]
        }   

        Solution::dijkstra(graph, k- 1)
    }
    pub fn dijkstra(graph: Vec<(i32, i32)>, src: i32) -> i32{

    }
}

fn main() {
    println!("Hello, world!");
}
