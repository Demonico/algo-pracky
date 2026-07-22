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