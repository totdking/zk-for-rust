/// Takes n number of steps to arrive at answer
/// O(n)
pub fn linear_time(n: u128) -> u128{
    let mut sum = 0;
    for i in 0..=n{
        sum +=i;
    }
    return sum;
}

/// Takes a fixed amount of steps to reach answer
/// O(1)
pub fn constant_time(n: u128) -> u128{
    let sum = n * (n+1) / 2 ;
    return sum;
}

/// O(n^2)
pub fn square(n: u32){
    for i in 0..n{
        for j in 0..n{
            println!("i: {:?}, j: {:?}", i, j);
        }
    }
}

/// O(log(n))
/// Recursive function
/// number of steps = log_2(n)
pub fn log_n(n: &mut u32) {
    *n = *n/2;
    if *n == 0{
        println!("Done");
        return;
    }
    // println!("End of iteration");
    log_n(n);
}

/// O(log(n))
/// iterative function
/// number of steps = log_2(n)
pub fn _log_n2(n: &mut u32){
    let mut i = 1;
    while *n > 1{
        *n = *n / 2;
        println!("End of iteration {:?}", i);
        i +=1;
    }
    println!("End of function");
}

/// O(log(n))
/// Binary search
pub fn binary_search(arr: &[u32], num: u32) {
    // Base case: if the array is empty, the number is not in it.
    if arr.is_empty() {
        println!("Number not found in array.");
        return;
    }

    let mid_index = arr.len() / 2;
    println!("mid index is {:?}", arr[mid_index]);

    if arr[mid_index] == num {
        println!("Binary search complete: Number found!");
    } else if arr[mid_index] > num {
        println!("The number is below the mid, searching left half.");
        println!("{:?}", &arr[..mid_index]);
        // Search in the left part of the array.
        binary_search(&arr[..mid_index], num);
    } else if arr[mid_index] < num {
        println!("The number is above the mid, searching right half.");
        println!("{:?}", &arr[mid_index + 1 ..]);
        // Search in the right part of the array.
        // We use mid_index + 1 to avoid getting stuck on the same element.
        binary_search(&arr[mid_index + 1..], num);
    }
}

/// O(nLog(n))
/// linear combined with logarithmic
pub fn n_log_n(n: & mut u32){
    let a = n.clone();
    while *n > 1{
       *n/=2;
       for i in 1..=a{
            println!("i is {:?}", i)
       }
    }
}

/// O(nLog(n))
/// Mergesort to sort an array to be ordered
/// it can be ordered or unordered
pub fn merge_sort(arr: &[u32]) -> Vec<u32>{
    // If the array is a single array, it has been sorted
    if arr.len() < 2{
        return arr.to_vec();
    }
    // mark the mid index to follow a divide and conquer approach
    let mid_index = arr.len() / 2 ;
    // Take the left array from the beginnng to the mid_index (exclusive range)
    let left_arr = &arr[..mid_index];
    // Take the right array as well
    let right_arr = &arr[mid_index..];
    // pass in the merge helper function to run the merge algo
    merge(&merge_sort(left_arr), &merge_sort(right_arr))
}
/// Helper function for the mergesort function
fn merge<'a>(left_arr: &'a [u32], right_arr: &'a [u32]) -> Vec<u32>{
    let mut res_arr: Vec<u32> = Vec::new();
    let mut left_index = 0;
    let mut right_index = 0;
    
    while left_index < left_arr.len() && right_index < right_arr.len() {
        if left_arr[left_index] < right_arr[right_index] {
            res_arr.push(left_arr[left_index]);
            left_index +=1;
        } else {
            res_arr.push(right_arr[right_index]);
            right_index += 1;
        }
    }

    // Extend the res_arr slice with the index remaining from both sides
    if left_index < left_arr.len(){
        res_arr.extend_from_slice(&left_arr[left_index..]);
    }
    if right_index < right_arr.len() {
        res_arr.extend_from_slice(&right_arr[right_index..]);
    }
    println!("result arr : {:?}", res_arr);
    res_arr
}

/// O(2^n)
/// 
/// Fibonnaci
pub fn fib(n: u32) -> u32{
    match n {
        0 => {return 0;},
        1 => {return 1;},
        _ => {return fib(n - 1) + fib(n - 2);}
    }
}

/// O(n!)
/// Factorial time
pub fn factorial(u: u32){
    if u == 0{
        println!("******");
        return;
    }
    for _ in 0..u{
        factorial(u - 1);

    }
}


pub mod big_o_notation;
use big_o_notation::*;

fn main() {
    // println!("Hello, world!");
    
    // Constant and linear time 
    // let a = linear_time(2);
    // let b = constant_time(2);

    // Quadratic time
    // let _ = square(100);

    // Logarithmic time
    // let mut a = 8;
    // let _ = log_n(&mut a);
    // let _ = log_n2(&mut a);
    // let ascending_arr= [1, 3, 23, 24, 25, 36, 40, 50, 60, 89, 99, 120, 145, 234];
    // let _ = binary_search(&ascending_arr, 23);
    // let arr_slice = &ascending_arr[3..];
    // println!("{:?}", arr_slice)

    // linearithmic time
    // let mut a = 8;
    // let _ = n_log_n(&mut a);
    // let array = [9, 3, 56, 199, 290, 12];
    // let _ = merge_sort(&array);

    // // Exponential
    // let fib = fib(6);
    // println!("{:?}", fib);
    
    // Fatorial
    // let _ = factorial(4);
}

