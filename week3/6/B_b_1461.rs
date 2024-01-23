// https://www.acmicpc.net/problem/1461

use std::{
    collections::btree_map::Values,
    io::{stdin, BufRead},
    result,
};

fn main() {
    let std = stdin();
    let mut buf = std.lock().lines();

    let numbers: Vec<usize> = buf
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|f| f.parse().unwrap())
        .collect();
    let n = numbers[0];
    let m = numbers[1];

    let mut books: Vec<i32> = Vec::new();
    let mut negative: Vec<i32> = Vec::new();
    let mut positive: Vec<i32> = Vec::new();

    books = buf
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|f| f.parse().unwrap())
        .collect();

    for book in books {
        if book > 0 {
            positive.push(book);
        }else{
            negative.push(book);
        }
    }

    negative.sort();
    positive.sort();
    positive.reverse();

    let mut distance: Vec<i32> = Vec::new();
    while !negative.is_empty() {
        let mut temp = 0;
        temp = negative.remove(0);
        for i in 1..m {
            if !negative.is_empty() {
                negative.remove(0);
            }
        }

        distance.push(-temp);
    }

    while !positive.is_empty() {
        let mut temp = 0;
        temp = positive.remove(0);
        for i in 1..m {
            if !positive.is_empty() {
                positive.remove(0);
            }
        }

        distance.push(temp);
    }

    let mut anwser = 0;
    distance.sort();

    for i in 0..distance.len() {
        if i == distance.len() - 1 {
            anwser += distance.get(i).unwrap();
        }else{
            anwser += (distance.get(i).unwrap() * 2);
        }
    }

    println!("{}", anwser);
}
