use std::{io::stdin, time::Instant};

fn find_pair_of_factors(n: i64, prime_factor_count: i64) -> (i64, i64) {
    let mut factor_1 = f64::powf(n as f64, 1.0 / prime_factor_count as f64).floor() as i64;
    let mut remainder = n % factor_1;
    let mut factor_2 = (n - remainder) / factor_1;
    while remainder % factor_2 != 0 {
        let reduced_remainder = remainder % factor_2;
        let new_factor_1 = factor_1 + remainder / factor_2 + 1;
        let new_factor_2 = factor_2 - 1;
        // In the equations this was a bit different. This is because the equations assumed you went through each of the values of a (factor_1).
        // The main difference here is that a is replaced with factor_1 - 1. This simplifies to what we have below:
        let new_remainder = reduced_remainder - factor_2 + new_factor_1;
        factor_1 = new_factor_1;
        factor_2 = new_factor_2;
        remainder = new_remainder;
    }
    // The long version which goes through every value of factor_1 will end when it is correct. However we aren't necessarily at that point yet.
    factor_1 += remainder / factor_2;
    (factor_1, factor_2)
}

fn main() {
    loop {
        let mut input = String::new();
        println!("Enter a number to find a pair of factors for:");
        stdin().read_line(&mut input).unwrap();
        let n = input.trim().parse::<i64>().unwrap();
        println!("How many prime factors are in it? Just use 2 if you don't know; it will still find factors");
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let prime_factor_count = input.trim().parse::<i64>().unwrap();
        let start = Instant::now();
        let (factor_1, factor_2) = find_pair_of_factors(n, prime_factor_count);
        let duration = start.elapsed();
        println!("{n} = {factor_1}*{factor_2}");
        println!("Found in {duration:?}");
    }
}
