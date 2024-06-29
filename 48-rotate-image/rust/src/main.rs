pub struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        // Transponse Matrix: Row to Column, Column to Row
        let row = matrix.len();
        let column = matrix[0].len();
        for i in 0..row {
            for j in (i + 1)..column {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }

        // Reversing the Rows of Transposed Matrix
        let col = matrix[0].len();
        for i in 0..matrix.len() {
            for j in 0..(col / 2) {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[i][col - 1 - j];
                matrix[i][col - 1 - j] = tmp;
            }
        }
    }
}

fn main() {
    let mut nums: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    println!("Rotation of Nums {:?}", nums);
    Solution::rotate(&mut nums);
    println!("Rotated Nums {:?}", nums);
}
