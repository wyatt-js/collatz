fn main() {
    // Wyatt Smith & Longiy Tsin
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
    return collatz_helper(n, 1)
}

fn collatz_helper(n: u64, length: u64) -> u64 {
    match n {
        1 => return length,
        n if n % 2 == 0 => collatz_helper(n / 2, length + 1),
        _ => collatz_helper(3 * n + 1, length + 1),
    }
}

fn longest_collatz(limit: u64) -> u64 {
    return longest_helper(limit, 1, 0, 0)
}

fn longest_helper(limit: u64, curr: u64, best: u64, best_length: u64) -> u64 {
    if curr > limit {
        return best
    }
    let curr_len = collatz_length(curr);
    if curr_len > best_length {
        longest_helper(limit, curr + 1, curr, curr_len)
    } else {
        longest_helper(limit, curr + 1, best, best_length)
    }
}