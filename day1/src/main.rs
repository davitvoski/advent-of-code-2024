use core::num;
use std::{env, fs, usize, vec};

// Problem 1
fn day1(mut list1: Vec<u32>, mut list2: Vec<u32>) -> u32 {
    let mut distance: u32 = 0;
    let mut l2 = list2.clone();

    list1.sort();
    list2.sort();

    for index in 0..list1.len() {
        distance += u32::abs_diff(list1[index], list2[index]);
    }

    return distance;
}

fn read_file(mut list1: Vec<u32>, mut l2: Vec<u32>) -> (Vec<u32>, Vec<u32>) {
    let fp: &str = "inputs.txt";
    let contents = fs::read_to_string(fp).expect("Should have been able to read the file");

    let all_paths: Vec<_> = contents
        .split_whitespace()
        .map(|x| x.parse::<i32>().expect("Not an Integer!"))
        .collect();

    let mut index = 0;

    while index < all_paths.len() {
        //println!("{}", all_paths[index].trim());
        let number: u32 = all_paths[index].try_into().unwrap();
        //.trim()
        //.parse::<u32>()
        //.expect("I dont know why it keeps failing");

        if index % 2 == 0 {
            list1.push(number);
        }

        if index % 2 != 0 {
            l2.push(number);
        }

        index += 1;
    }

    return (list1, l2);
}

fn main() {
    println!("Hello, world!");

    env::set_var("RUST_BACKTRACE", "1");

    let l1: Vec<u32> = Vec::new();
    let l2: Vec<u32> = Vec::new();
    //let res = read_file(&l1, &l2);

    let result = read_file(l1, l2);

    println!("{}", day1(result.0, result.1));
    // println!("{:?}", result.0);

    //   println!("----------------------------------------------------------------");
    //    println!("{:?}", result.1);
}
