use std::io::{stdin, BufRead};

fn main() {
    let std= stdin();
    let mut buf= std.lock().lines();
    
    // n
    let n: usize= buf.next().unwrap().unwrap().trim().parse().unwrap();

    // n and so on
    let numbers: Vec<usize>= buf.next().unwrap().unwrap().trim().split_whitespace().map(|f| f.parse().unwrap()).collect();
    // let n= numbers[0];

    let mut list: Vec<i32>= Vec::new();
    for _ in 0..n {
        // 0 1 2
        let values: Vec<i32>= buf.next().unwrap().unwrap().trim().split_whitespace().map(|f| f.parse().unwrap()).collect();
        list.push(values[0]);

        // abcd
        let value= buf.next().unwrap().unwrap().trim().to_string();
        list.push(value);

        // abcd abcd
        let values: Vec<String>= buf.next().unwrap().unwrap().trim().split_whitespace().map(|f| f.to_string()).collect();
        list.push(values[0]);
    }
}
