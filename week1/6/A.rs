// https://www.acmicpc.net/problem/2002
// test case 1,2,3은 통과
// 1: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=28d4f848b9a54dd25882fce095a0e9a4
// 2: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=f3f2b0bace0d9a06be639172d25ae630
// 3: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=06d5aee3653f5375d794edeab7a81acc

use std::io::{stdin, BufRead};
fn main() {
    let mut in_cars: Vec<String>= Vec::new();
    let mut out_cars: Vec<String>= Vec::new();
    
    let std= stdin();
    let mut buf= std.lock().lines();
    let input= buf.next().unwrap().unwrap();
    
    let n: u32= input.parse().unwrap();
    for _ in 0..n{
        let car_nubmer_str= buf.next().unwrap().unwrap();
        in_cars.push(car_nubmer_str);
    }
    for _ in 0..n{
        let car_nubmer_str= buf.next().unwrap().unwrap();
        out_cars.push(car_nubmer_str);
    }
    
    let car_size= out_cars.len();
    if car_size == 1{
        println!("0");
    }

    let mut cnt= 0;
    for (i, in_car) in in_cars.iter().enumerate(){
        for (j, out_car) in out_cars.iter().enumerate(){
            if in_car == out_car && i != j{
                if i> j {
                    cnt+=1;
                }
            }
        }
    }
    println!("{}", cnt);
}
