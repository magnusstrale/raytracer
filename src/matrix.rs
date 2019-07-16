use core::ops;
use super::tuple::Tuple;

#[derive(Debug, Clone)]
struct Row {
    inner: Vec<f64>
}

impl ops::Index<usize> for Row {
    type Output = f64;
    fn index(&self, col: usize) -> &Self::Output {
        &self.inner[col]
    }
}

impl PartialEq for Row {
    fn eq(&self, other: &Self) -> bool {
        const EPS: f64 = 0.00001;
        let size = self.inner.len();
        size == other.inner.len() &&
        (0..size).all(|col| (self[col] - other[col]).abs() < EPS)
    }
}

#[derive(Debug, Clone)]
struct Matrix {
    inner: Vec<Row>,
}

impl ops::Mul<Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Matrix {
        let mut m = Matrix::new_empty();
        let size = self.inner.len();
        for row in 0..size {
            for col in 0..size {
                let a = (0..size).map(|i| self[row][i] * rhs[i][col]).sum();
                m.set(row, col, a);
            }
        }
        m
    }
}

impl ops::Mul<Tuple> for Matrix {
    type Output = Tuple;
    fn mul(self, rhs: Tuple) -> Tuple {
        Tuple::new(
            self.tuple(0).dot(&rhs),
            self.tuple(1).dot(&rhs),
            self.tuple(2).dot(&rhs),
            self.tuple(3).dot(&rhs))
    }
}

impl ops::Index<usize> for Matrix {
    type Output = Row;
    fn index(&self, row: usize) -> &Self::Output {
        &self.inner[row]
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        let size = self.inner.len();
        size == other.inner.len() &&
        (0..size).all(|row| self[row] == other[row])
    }
}

impl Matrix {
    const EMPTY_ROW:  [f64; 4] = [0.0, 0.0, 0.0, 0.0];
    const EMPTY_ROW3: [f64; 3] = [0.0, 0.0, 0.0];
    const EMPTY_ROW2: [f64; 2] = [0.0, 0.0];

    fn new(row0: [f64; 4], row1: [f64; 4], row2 : [f64; 4], row3 : [f64; 4]) -> Matrix
    {
        Matrix { inner: vec![ Row { inner: row0.to_vec()}, Row { inner: row1.to_vec()}, Row { inner: row2.to_vec()}, Row { inner: row3.to_vec()}]}
    }

    fn new3(row0: [f64; 3], row1: [f64; 3], row2 : [f64; 3]) -> Matrix
    {
        Matrix { inner: vec![ Row { inner: row0.to_vec()}, Row { inner: row1.to_vec()}, Row { inner: row2.to_vec()}]}
    }

    fn new2(row0: [f64; 2], row1: [f64; 2]) -> Matrix
    {
        Matrix { inner: vec![ Row { inner: row0.to_vec()}, Row { inner: row1.to_vec()}]}
    }

    fn new_empty() -> Matrix {
        Matrix::new(Matrix::EMPTY_ROW, Matrix::EMPTY_ROW, Matrix::EMPTY_ROW, Matrix::EMPTY_ROW)
    }

    fn new_empty3() -> Matrix {
        Matrix::new3(Matrix::EMPTY_ROW3, Matrix::EMPTY_ROW3, Matrix::EMPTY_ROW3)
    }

    fn new_empty2() -> Matrix {
        Matrix::new2(Matrix::EMPTY_ROW2, Matrix::EMPTY_ROW2)
    }

    fn identity_matrix() -> Matrix {
        Matrix { inner: vec![
            Row { inner: vec![1.0, 0.0, 0.0, 0.0] }, 
            Row { inner: vec![0.0, 1.0, 0.0, 0.0] },
            Row { inner: vec![0.0, 0.0, 1.0, 0.0] },
            Row { inner: vec![0.0, 0.0, 0.0, 1.0] } ] }
    }

    fn set(&mut self, row: usize, col: usize, value: f64) {
        self.inner[row].inner[col] = value;
    }

    fn tuple(&self, row: usize) -> Tuple {
        let r = &self[row];
        Tuple::new(r[0], r[1], r[2], r[3])
    }

    fn transpose(&self) -> Matrix {
        let mut m = Matrix::new_empty();
        let size = self.inner.len();
        for row in 0..size {
            for col in 0..size {
                m.set(col, row, self[row][col]);
            }
        }
        m
    }

    fn determinant2(&self) -> f64 {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }

    fn submatrix(&self, row: usize, col: usize) -> Matrix {
        let size = self.inner.len();
        let mut m = match size {
            4 => Option::Some(Matrix::new_empty3()),
            3 => Option::Some(Matrix::new_empty2()),
            _ => Option::None
        }.expect("Invalid matrix size, only 3x3 and 4x4 supported");
        let mut r_new = 0;
        
        for r in 0..size {
            if r == row { continue; }
            let mut c_new = 0;
            for c in 0..size {
                if c == col { continue; }
                m.set(r_new, c_new, self[r][c]);
                c_new += 1;
            }
            r_new += 1;
        }
        m
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_4x4_matrix()
    {
        let m = Matrix::new(
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5]);
        assert_eq!(1.0, m[0][0]);
        assert_eq!(4.0, m[0][3]);
        assert_eq!(5.5, m[1][0]);
        assert_eq!(7.5, m[1][2]);
        assert_eq!(11.0, m[2][2]);
        assert_eq!(13.5, m[3][0]);
        assert_eq!(15.5, m[3][2]);
    }

    #[test]
    fn construct_3x3_matrix()
    {
        let m: Matrix = Matrix::new3(
            [-3.0, 5.0, 0.0],
            [1.0, -2.0, -7.0],
            [0.0, 1.0, 1.0]);
        assert_eq!(-3.0, m[0][0]);
        assert_eq!(-2.0, m[1][1]);
        assert_eq!(1.0, m[2][2]);
    }

    #[test]
    fn construct_2x2_matrix()
    {
        let m: Matrix = Matrix::new2(
            [-3.0, 5.0],
            [1.0, -2.0]);
        assert_eq!(-3.0, m[0][0]);
        assert_eq!(5.0, m[0][1]);
        assert_eq!(1.0, m[1][0]);
        assert_eq!(-2.0, m[1][1]);
    }

    #[test]
    fn matrix_equality_identical_matrices() {
        let a = Matrix::new(
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0]);
        let b = Matrix::new(
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0]);

        assert_eq!(a, b);
    }

    #[test]
    fn matrix_equality_different_matrices() {
        let a = Matrix::new(
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0]);
        let b = Matrix::new(
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 1.0]);

        assert_ne!(a, b);
    }

    #[test]
    fn multiplying_two_matrices() {
        let a = Matrix::new (
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0]);
        let b = Matrix::new(
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0]);
        let expected = Matrix::new(
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0]);

        let result = a * b;
        assert_eq!(expected, result);
    }

    #[test]
    fn tuple_from_matrix_row()
    {
        let a = Matrix::new (
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0]);
        let b = Tuple::new(5.0, 4.0, 3.0, 2.0);

        assert_eq!(b, a.tuple(3));
    }    

    #[test]
    fn multiply_matrix_by_tuple()
    {
        let a = Matrix::new(
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0]);
        let b = Tuple::new(1.0, 2.0, 3.0, 1.0);
        let expected = Tuple::new(18.0, 24.0, 33.0, 1.0);
        
        assert_eq!(expected, a * b);
    }

    #[test]
    fn multiply_matrix_by_identity_matrix()
    {
        let a = Matrix::new(
            [0.0, 1.0, 2.0, 8.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0]);
        
        assert_eq!(a, a.clone() * Matrix::identity_matrix());
    }

    #[test]
    fn transpose_matrix()
    {
        let a = Matrix::new(
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0]);        
        let expected = Matrix::new(
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0]);
        
        assert_eq!(expected, a.transpose());
    }

    #[test]
    fn transpose_identity_matrix()
    {
        assert_eq!(Matrix::identity_matrix(), Matrix::identity_matrix().transpose());
    }

    #[test]
    fn determinant_2x2_matrix()
    {
        let a = Matrix::new2([1.0, 5.0], [-3.0, 2.0]);
        assert_eq!(17.0, a.determinant2());
    }

    #[test]
    fn submatrix_of_3x3_is_2x2_matrix() {
        let a = Matrix::new3(
            [1.0, 5.0, 0.0],
            [-3.0, 2.0, 7.0],
            [0.0, 6.0, -3.0]);
        let expected = Matrix::new2([-3.0, 2.0], [0.0, 6.0]);
        assert_eq!(expected, a.submatrix(0, 2));
    }

    #[test]
    fn submatrix_of_4x4_is_3x3_matrix() {
        let a = Matrix::new(
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0]);
        let expected = Matrix::new3(
            [-6.0, 1.0, 6.0], 
            [-8.0, 8.0, 6.0], 
            [-7.0, -1.0, 1.0]);
        assert_eq!(expected, a.submatrix(2, 1));
    }

    #[test]
    #[should_panic]
    fn submatrix_of_2x2_matrix_shold_panic()
    {
        let a = Matrix::new2(
            [-6.0, 2.0], 
            [-7.0, 1.0]);
        a.submatrix(1, 1);
    }
}