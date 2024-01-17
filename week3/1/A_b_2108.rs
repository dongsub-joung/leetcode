// https://www.acmicpc.net/problem/2108

// uncorrected
use std::{io::{stdin, BufRead}, collections::{HashMap, hash_map}, result};

pub fn quick_sort(arr: &mut Vec<i32>) {
    let len = arr.len();
    _quick_sort(arr, 0, (len - 1) as isize);
}

fn _quick_sort(arr: &mut Vec<i32>, low: isize, high: isize) {
    if low < high {
        let p = partition(arr, low, high);
        _quick_sort(arr, low, p - 1);
        _quick_sort(arr, p + 1, high);
    }
}

fn partition(arr: &mut Vec<i32>, low: isize, high: isize) -> isize {
    let pivot = high as usize;
    let mut store_index = low - 1;
    let mut last_index = high;

    loop {
        store_index += 1;
        while arr[store_index as usize] < arr[pivot] {
            store_index += 1;
        }
        last_index -= 1;
        while last_index >= 0 && arr[last_index as usize] > arr[pivot] {
            last_index -= 1;
        }
        if store_index >= last_index {
            break;
        } else {
            arr.swap(store_index as usize, last_index as usize);
        }
    }
    arr.swap(store_index as usize, pivot as usize);
    store_index
}					

fn main() {
    let mut n_list: Vec<i32>= Vec::new();
    let mut results: Vec<i32>= Vec::new();

    let std= stdin();
    let mut buf= std.lock().lines();
    let n: usize= buf.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..n{
        let number: i32= buf.next().unwrap().unwrap().parse().unwrap();
        n_list.push(number);
    }

    // attempt to use a non-constant value in a constant non-constant value
    // const N_LIST: Vec<i32>= n_list;

    // 산술평균
    let sumed: i32= n_list.clone().iter().sum();
    results.push(sumed % (n as i32));

    // 중앙값
    quick_sort(&mut n_list);
    let center: i32= (n_list.len() as i32) / 2;
    results.push(n_list[center as usize].clone());

    // 최빈값
    let mut hashmap: HashMap<i32, usize>= HashMap::new();
    for (_, e) in n_list.clone().iter().enumerate() {
        if hashmap.contains_key(&e) {
            let value= hashmap.get(e).unwrap().clone() +1;
            hashmap.insert(*e, value);
        }
        hashmap.insert(e.clone(), 1);
    }
    let mut mini= usize::MAX;
    let mut min_idx= 0;
    for (idx , e) in hashmap.clone() {
        if mini > e {
            mini= e;
            min_idx= idx;
        }
    }
    let val= mini as i32;
    let hashmap_result= hashmap.get(&min_idx).unwrap().clone() as i32;
    results.push(hashmap_result);

    // 범위
    let n_min= n_list.iter().min().unwrap();
    let n_max= n_list.iter().max().unwrap();
    results.push(n_max-n_min);

    println!("{:?}", results);
}
