//! Basic examples of using infinite arrays

use infinite_arrays::*;

fn main() {
    println!("=== Infinite Arrays Examples ===\n");

    // Example 1: Ones
    println!("1. Infinite array of ones:");
    let ones: Ones<f64> = Ones::new();
    println!("   ones.get(0) = {}", ones.get(0));
    println!("   ones.get(100) = {}", ones.get(100));
    println!();

    // Example 2: Range
    println!("2. Infinite range (OneToInf):");
    let range: OneToInf<usize> = OneToInf::new();
    println!("   range.get(0) = {}", range.get(0));
    println!("   range.get(1) = {}", range.get(1));
    println!("   range.get(99) = {}", range.get(99));
    println!();

    // Example 3: Cumulative sum
    println!("3. Cumulative sum of ones:");
    let ones: Ones<f64> = Ones::new();
    let cumsum_result = cumsum(ones);
    println!("   cumsum.get(0) = {}", cumsum_result.get(0));
    println!("   cumsum.get(1) = {}", cumsum_result.get(1));
    println!("   cumsum.get(9) = {}", cumsum_result.get(9));
    println!();

    // Example 4: Broadcasting
    println!("4. Broadcasting (multiply by 2):");
    let ones: Ones<f64> = Ones::new();
    let doubled = broadcast(ones, |x| x * 2.0);
    println!("   doubled.get(0) = {}", doubled.get(0));
    println!("   doubled.get(100) = {}", doubled.get(100));
    println!();

    // Example 5: Scalar addition
    println!("5. Scalar addition (ones + 2):");
    let ones: Ones<f64> = Ones::new();
    let result = add_scalar(ones, 2.0);
    println!("   result.get(0) = {}", result.get(0));
    println!("   result.get(100) = {}", result.get(100));
    println!();

    // Example 6: Cached array
    println!("6. Cached array (mutable):");
    let ones: Ones<f64> = Ones::new();
    let mut cached = CachedArray::new(ones);
    println!("   cached.get(0) = {}", cached.get(0));
    cached.set(0, 3.0);
    println!("   After setting cached[0] = 3.0:");
    println!("   cached.get(0) = {}", cached.get(0));
    println!("   cached.get(1) = {}", cached.get(1));
    println!();

    // Example 7: Array from function
    println!("7. Infinite array from function (i * 2):");
    let arr = InfiniteArrayFromFn::new(|i| i * 2);
    println!("   arr.get(0) = {}", arr.get(0));
    println!("   arr.get(1) = {}", arr.get(1));
    println!("   arr.get(5) = {}", arr.get(5));
    println!();

    // Example 8: Iteration
    println!("8. Iteration over ones (first 5 elements):");
    let ones: Ones<f64> = Ones::new();
    let mut iter = ones.iter();
    for _ in 0..5 {
        println!("   iter.next() = {:?}", iter.next());
    }
}

