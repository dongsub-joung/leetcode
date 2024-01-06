// https://github.com/raul-sauco/coding-challenges/blob/main/leetcode/the-k-weakest-rows-in-a-matrix.rs

use std::collections::BinaryHeap;

pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    fn get_row_solider_count(row: &Vec<i32>) -> usize{
        if row[0] == 0{
            return 0;
        }else if row[row.len() -1] == 1{
            return row.len();
        }

        let (mut l, mut r)= (0, row.len());
        let mut mid;
        while r - l > 1 {
            mid= (l+r) / 2;
            if row[mid] == 0{
                r= mid;
            }else{
                l= mid;
            }
        }

        r
    }
    
    let k= k as usize;
    let mut weakset: BinaryHeap<(usize, usize)>= BinaryHeap::with_capacity(k+1);
    let mut soldier_count;

    for row_idx in 0..mat.len(){
        soldier_count= get_row_solider_count(&mat[row_idx]);
        weakset.push((soldier_count, row_idx));

        if weakset.len() > k {
            weakset.pop();
        }
    }  

    let mut res= (0..k)
        .map(|_| weakset.pop().unwrap().1 as i32)
        .collect::<Vec<_>>();

    res.reverse();
    
    res  
}


#[test]
fn test1() {
    let input= vec![
        vec![1, 1, 0, 0, 0],
        vec![1, 1, 1, 1, 0],
        vec![1, 0, 0, 0, 0],
        vec![1, 1, 0, 0, 0],
        vec![1, 1, 1, 1, 1],
    ];
    let k= 3;
    let expect= vec![2, 0, 3];

    let result= k_weakest_rows(input, k);
    assert_eq!(expect, result)
}

#[test]

fn test2() {
    let input=vec![
            vec![1, 0, 0, 0],
            vec![1, 1, 1, 1],
            vec![1, 0, 0, 0],
            vec![1, 0, 0, 0],
        ];
    let k= 2;
    let expect= vec![0, 2];

    let result= k_weakest_rows(input, k);
    assert_eq!(expect, result)
}
