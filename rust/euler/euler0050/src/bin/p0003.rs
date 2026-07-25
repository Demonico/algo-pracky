use common::sieve_of_eratosthenes;

/*
    Brute Force
    find all primes up to the square root of the input and return the largest
    Sieve of Eratosthenes will efficiently find all primes up to a target
*/

fn solve_largest_prime_factor(input: i64) -> i64 {
    let prime_list: Vec<i64> = sieve_of_eratosthenes(input.isqrt() as usize);
    for &prime in prime_list.iter().rev() {
        if input % prime == 0 {
            return prime;
        }
    }
    0
}

fn main() {
    let input:i64 = 600851475143;
    let res = solve_largest_prime_factor(600851475143);
    println!("{} is the largest prime factor of {}", res, input)
}
