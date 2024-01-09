// https://www.acmicpc.net/problem/4949
// 엣지 케이스 처리를 못하겠다.

use std::io::{stdin, BufRead};
fn main() {
    let std= stdin();
    let mut buf= std.lock().lines();
    
    let mut strs: Vec<String>= Vec::new();
    let mut big:Vec<char>= Vec::new();
    let mut small:Vec<char>= Vec::new();
    let mut answers: Vec<&str>= Vec::new();

    loop{
        let inputs= buf.next().unwrap().unwrap();
        if inputs == "."{
            break;
        }

        let chars= inputs.chars();

        for c in chars {
            if c == '('{
                small.push(c);
            }else if c == '[' {
                big.push(c);
            }

            if c == ')'{
                small.pop();
            }else if c == ']'{
                big.pop();
            }
        }

        if !small.is_empty(){
            answers.push("no");
        }else if !big.is_empty(){
            answers.push("no")
        }else{
            answers.push("yes");
        }
    }
    
    for answer in answers{
        println!("{}", answer);
    }
}


// yes
// yes
// no
// no
// no
// no
// no