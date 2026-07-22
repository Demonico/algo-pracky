fn brute_force(input: i64) -> i64 {
    let mut res: i64 = 0;
    for x in 1..input {
        if x % 3 == 0 || x % 5 == 0 {
            res += x;
        }
    }
    return res;
}

fn main() {
    println!("{}",brute_force(1000))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_brute_force() {
        assert_eq!(brute_force(10),23);
        assert_eq!(brute_force(100),2318);
    }
}