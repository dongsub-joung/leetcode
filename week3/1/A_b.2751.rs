use std::io::{stdin, BufRead};
fn main() {
    let mut list: Vec<i32>= Vec::new();
    
    let std= stdin();
    let mut buf= std.lock().lines();
    
    let n: usize= buf.next().unwrap().unwrap().trim().parse().unwrap();
    for _ in 0..n{
        let value: i32= buf.next().unwrap().unwrap().trim().parse().unwrap();
        list.push(value);
    }

    // @todo make the sort algori
    list.sort();

    for e in list {
        println!("{}", e);
    }
}
