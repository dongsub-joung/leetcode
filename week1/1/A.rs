// From: A Po
// Author: Chris Chung
// Link: https://chungchris.github.io/2020/07/13/software/leetcode/Group-Anagrams/#toc-heading-4

use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let base= 'a' as u32;
    let mut collect: HashMap<String, Vec<String>>= HashMap::new();
    for s in strs{
        let mut count: [i32; 26]= [0; 26];
        for c in s.chars(){
            count[(c as u32 - base) as usize ] += 1;
        }

        let id= format!("{:?}", count);
        match collect.get_mut(&id) {
            Some(r) => { r.push(s); }, // r is auto deref
            None => { collect.insert(id, vec![s]); }
        }
    }

    let mut ans:Vec<Vec<String>>= vec![];
    for (_ , v) in collect.drain(){
        ans.push(v);
    }

    ans
}

fn main(){
    let result= group_anagrams(vec!["eat".to_string(),"tea".to_string(),"tan".to_string()
        ,"ate".to_string(),"nat".to_string(),"bat".to_string()]);
    println!("{:?}", result)
}
