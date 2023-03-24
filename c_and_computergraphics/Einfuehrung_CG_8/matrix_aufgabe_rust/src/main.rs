use matrix::Matrix;

fn main() {
    let m1 = Matrix::new_from_values(vec![vec![1f32, 2f32, 3f32], vec![4f32, 5f32, 6f32]]);
    let m2 = Matrix::new_from_values(vec![vec![1f32, 2f32], vec![3f32, 4f32], vec![5f32, 6f32]]);

    let m = m1 * m2;

    println!("{}", m);
}
