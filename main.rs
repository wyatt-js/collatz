fn main() {
    // Compile with: rustc main.rs && ./main
    // Known-answer checks — these will panic if your functions are wrong
    assert_eq!(collatz_length(1), 1, "length of 1 should be 1");
    assert_eq!(collatz_length(6), 9, "length of 6 should be 9");
    assert_eq!(collatz_length(27), 112, "length of 27 should be 112");
    println!("collatz_length checks passed.");
    let answer = longest_collatz(1_000_000);
    assert_eq!(answer, 837799, "longest below 1M should be 837799");
    println!("longest_collatz(1_000_000) = {}", answer);
    println!("All checks passed.");
   }
   fn collatz_length(n: u64) -> u64 {
    todo!()
   }
   fn longest_collatz(limit: u64) -> u64 {
    todo!()
   }