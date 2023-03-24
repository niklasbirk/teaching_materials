use std::fmt::{Display, Formatter};
use std::ops::{Add, Mul};

pub struct Matrix {
    m_type: (usize, usize),
    values: Vec<Vec<f32>>
}

impl Matrix {
    pub fn new((m, n): (usize, usize)) -> Self {
        let mut m_values = Vec::new();

        for _i in 0..m {
            m_values.push(vec![0f32; n]);
        }

        Self {
            m_type: (m, n),
            values: m_values
        }
    }

    pub fn new_from_values(m_values: Vec<Vec<f32>>) -> Self {
        Self {
            m_type: (m_values.len(), m_values.get(0).unwrap().len()),
            values: m_values
        }
    }

    fn add_matrices(&self, m: &Self) -> Self {
        let mut values = Vec::new();

        for i in 0..self.m_type.0 {
            let mut row = vec![];

            for j in 0..self.m_type.1 {
                let a = self.values.get(i).unwrap().get(j).unwrap();
                let b = m.values.get(i).unwrap().get(j).unwrap();

                row.push(a + b);
            }

            values.push(row);
        }

        Self::new_from_values(values)
    }

    pub fn multiply_with_scalar(&self, c: f32) -> Self {
        let mut values = Vec::new();

        for i in 0..self.m_type.0 {
            let mut row = vec![];

            for j in 0..self.m_type.1 {
                let a = self.values.get(i).unwrap().get(j).unwrap();

                row.push(c * a);
            }

            values.push(row);
        }

        Self::new_from_values(values)
    }

    fn multiply(&self, m: &Self) -> Self {
        let mut values = Vec::new();

        for i in 0..self.m_type.0 {
            let mut row = vec![];

            for j in 0..m.m_type.1 {
                let mut c = 0f32;

                for k in 0..self.m_type.1 {
                    let a = self.values.get(i).unwrap().get(k).unwrap();
                    let b = m.values.get(k).unwrap().get(j).unwrap();

                    c += a * b;
                }

                row.push(c);
            }

            values.push(row);
        }

        Self::new_from_values(values)
    }
}

impl Add for Matrix {
    type Output = Matrix;

    fn add(self, m2: Self) -> Self::Output {
        self.add_matrices(&m2)
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, m2: Self) -> Self::Output {
        self.multiply(&m2)
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.values)
    }
}

#[cfg(test)]
mod tests {
    use crate::Matrix;

    #[test]
    fn should_return_2_2_matrix_with_zeros() {
        let matrix = Matrix::new((2, 2));

        assert_eq!(matrix.m_type, (2, 2));
        assert_eq!(matrix.values, vec![vec![0f32, 0f32], vec![0f32, 0f32]]);
    }

    #[test]
    fn should_return_2_2_matrix_with_values() {
        let matrix = Matrix::new_from_values(vec![vec![0f32, 1f32], vec![2f32, 3f32]]);

        assert_eq!(matrix.m_type, (2, 2));
        assert_eq!(matrix.values, vec![vec![0f32, 1f32], vec![2f32, 3f32]]);
    }

    #[test]
    fn should_add_two_matrices() {
        let m1 = Matrix::new_from_values(vec![vec![0f32, 1f32], vec![2f32, 3f32]]);
        let m2 = Matrix::new_from_values(vec![vec![4f32, 5f32], vec![6f32, 7f32]]);

        let m3 = m1.add_matrices(&m2);

        assert_eq!(m3.m_type, (2, 2));
        assert_eq!(m3.values, vec![vec![4f32, 6f32], vec![8f32, 10f32]]);
    }

    #[test]
    fn should_add_two_matrices_with_op() {
        let m1 = Matrix::new_from_values(vec![vec![0f32, 1f32], vec![2f32, 3f32]]);
        let m2 = Matrix::new_from_values(vec![vec![4f32, 5f32], vec![6f32, 7f32]]);

        let m3 = m1 + m2;

        assert_eq!(m3.m_type, (2, 2));
        assert_eq!(m3.values, vec![vec![4f32, 6f32], vec![8f32, 10f32]]);
    }

    #[test]
    fn should_multiply_matrix_with_2() {
        let m1 = Matrix::new_from_values(vec![vec![0f32, 1f32], vec![2f32, 3f32]]);

        let m = m1.multiply_with_scalar(2f32);

        assert_eq!(m.m_type, (2, 2));
        assert_eq!(m.values, vec![vec![0f32, 2f32], vec![4f32, 6f32]]);
    }

    #[test]
    fn should_multiply_matrices() {
        let m1 = Matrix::new_from_values(vec![vec![1f32, 2f32, 3f32], vec![4f32, 5f32, 6f32]]);
        let m2 = Matrix::new_from_values(vec![vec![1f32, 2f32], vec![3f32, 4f32], vec![5f32, 6f32]]);

        let m = m1.multiply(&m2);

        assert_eq!(m.m_type, (2, 2));
        assert_eq!(m.values, vec![vec![22f32, 28f32], vec![49f32, 64f32]]);
    }

    #[test]
    fn should_multiply_matrices_with_op() {
        let m1 = Matrix::new_from_values(vec![vec![1f32, 2f32, 3f32], vec![4f32, 5f32, 6f32]]);
        let m2 = Matrix::new_from_values(vec![vec![1f32, 2f32], vec![3f32, 4f32], vec![5f32, 6f32]]);

        let m = m1 * m2;

        assert_eq!(m.m_type, (2, 2));
        assert_eq!(m.values, vec![vec![22f32, 28f32], vec![49f32, 64f32]]);
    }
}
