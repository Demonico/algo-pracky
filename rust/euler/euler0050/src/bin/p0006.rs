fn solve_sum_square_diff(end: i64) -> i64 {
    let sum_of_squares: i64 = (1..=end).map(|x| x.pow(2)).sum();
    let square_of_sum: i64 = (1..=end).sum::<i64>().pow(2);
    // println!("sum_of_squares is {}, square_of_sum is {}", sum_of_squares, square_of_sum);
    (sum_of_squares - square_of_sum).abs()
}

fn main() {
    const VAL:i64 = 100;
    let res = solve_sum_square_diff(VAL);
    println!("result of first {} is {}", VAL, res)
}

