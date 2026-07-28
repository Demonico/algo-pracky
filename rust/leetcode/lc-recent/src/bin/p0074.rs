// Binary search with a sorted m x n 2D grid
// start in top right or bottom left corner
// binary condition removes a row or col in each iteration
fn search_matrix(matrix: &Vec<Vec<i32>>, target: i32) -> bool {
    let mut row:usize = 0;
    let mut col:usize = matrix[0].len() - 1;

    while row < matrix.len() {
        if matrix[row][col] == target {
            return true
        } else if matrix[row][col] < target {
            row += 1;
        } else {
            match col.checked_sub(1) {
                Some(c) => col = c,
                None => return false,
            }
        }
    }
    false
}

fn main() {
    let input: Vec<Vec<i32>> = vec![
        vec![11,13,15,17],
        vec![101,111,116,120],
        vec![123,130,134,160]
    ];
    fn results(matrix: &Vec<Vec<i32>>, target: i32) {
        let result = search_matrix(matrix, target);
        let res = if result {"is"} else {"is not"};
        println!("{target} {res} in the matrix.")
    }
    results(&input, 6);
    results(&input,123);
}