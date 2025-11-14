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
    println!("End of iteration");
    log_n(n);
}

/// O(log(n))
/// iterative function
/// number of steps = log_2(n)
pub(crate) fn _log_n2(n: &mut u32){
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
/// Mergesort
pub fn merge_sort(arr: &[u32], n: u32){
    if arr.len() < 2{
        println!("Array has being sorted");
    }
    let mid_index = arr.len() / 2 ;
    let left_arr = &arr[..mid_index];
    let right_arr = &arr[mid_index..];
}
/// Helper function for the mergesort function
fn merge(left_arr: &[u32], right_arr: &[u32]){
    let mut res_arr: Vec<u32> = Vec::new();
    let mut left_index = 0;
    let mut right_index = 0;

}