// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

/// Returns a prime vector up to and including `limit`, where index i is
/// true iff i is prime.
pub fn sieve_of_eratosthenes(limit: usize) -> Vec<i64> {
    let mut is_prime = vec![true; limit+1];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..=limit {
        if is_prime[i] {
            for j in (i*i..=limit).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    is_prime
        .iter()
        .enumerate()
        .filter(|&(_, &p)| p)
        .map(|(i, _)| i as i64)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sieve_of_eratosthenes() {
        let result = sieve_of_eratosthenes(100);
        assert!(result.contains(&53));
        assert_eq!(*result.last().unwrap_or(&0), 97)
    }
}
