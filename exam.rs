pub fn reverse_it(v: i32) -> String {
    let mut s=String::new();

    if v<0 {
        s.push('-');
    
    }
   
    let v2=v.abs().to_string();
    let v1=v2.chars().rev().collect::<String>();

    s.push_str(&v1);
    s.push_str(&v2);
}




use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    *h.values().max().unwrap()
}

pub fn parts_sums(arr: &[u64]) -> Vec<u64>{

    let mut new_arr=Vec::new();

    let mut sum=0;
    for i in 0..arr.len(){
        sum+=arr[i];
        new_arr.push(sum);
    }
    new_arr.push(0);
   new_arr.reverse();

   return new_arr;
}




pub fn inv_pyramid(v: String, i: u32) -> Vec<String> {

    let mut new_str=Vec::new();

    for j in 1..=i{
        new_str.push(String::from(" ").repeat(j as usize)+&v.repeat(j as usize));
    }

    for j in (1..=i).rev() {
        if i<2{ continue; }
        new_str.push(String::from(" ").repeat(j as usize)+&v.repeat(j as usize));
    }
        
    new_str
}



pub fn insertion_sort(slice: &mut [i32], steps: usize) {

    for i in 1..=steps {
        let mut j=i;
        while j>0 && slice[j] < slice[j-1] {
            slice.swap(j, j-1);
            j-=1;
        }
    }
}

pub fn rpn(expression: &str) {

    let tokens: Vec<&str> = expression.trim().split_whitespace().collect();

    let mut stack: Vec<i64> = Vec::new();

    for token in tokens {
        match token {
            "+" => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(a + b);
                } else {
                    println!("Error");
                    return;
                }
            }
            "-" => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(a - b);
                } else {
                    println!("Error");
                    return;
                }
            }
            "*" => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(a * b);
                } else {
                    println!("Error");
                    return;
                }
            }
            "/" => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    if b == 0 {
                        println!("Error");
                        return;
                    }
                    stack.push(a / b);
                } else {
                    println!("Error");
                    return;
                }
            }
            "%" => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    if b == 0 {
                        println!("Error");
                        return;
                    }
                    stack.push(a % b);
                } else {
                    println!("Error");
                    return;
                }
            }
            _ => {
                if let Ok(num) = token.parse::<i64>() {
                    stack.push(num);
                } else {
                    println!("Error");
                    return;
                }
            }
        }
    }

    if stack.len() != 1 {
        println!("Error");
    } else {
        println!("{}", stack[0]);
    }
}


