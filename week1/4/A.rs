// https://github.com/doocs/leetcode/blob/main/solution/1400-1499/1464.Maximum%20Product%20of%20Two%20Elements%20in%20an%20Array/README_EN.md

fn max_product(nums: Vec<i32>) -> i32{
    let mut max= 0;
    let mut submax= 0;

    for &num in nums.iter(){
        if num > max{
            submax= max;
            max= num;
        }else if num>submax{
            submax= num;
        }
    }

    (max - 1) * (submax - 1)
}


#[test]
fn test1() {
    let result= max_product(vec![3,4,5,2]);
    assert_eq!(12, result)
}
