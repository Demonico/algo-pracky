fn solve_even_fib_sum(input: i64) -> i64 {
    let mut res: i64 = 0;
    let mut prev1: i64 = 1;
    let mut prev2: i64 = 1;

    while prev1 + prev2 <= input {
        let nxt = prev1 + prev2;
        if nxt % 2 == 0 {
            res += nxt;
        }
        prev2 = prev1;
        prev1 = nxt;
    }
    return res;
}

fn main() {
    println!("{}", solve_even_fib_sum(4000000))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve_even_fib_sum() {
        assert_eq!(solve_even_fib_sum(100), 44)
    }
}
