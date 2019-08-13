use core::ops;
use super::tuple::Tuple;

#[derive(Debug, Copy, Clone)]
pub struct Row {
    inner: [f64; 4],
    size: usize
}

impl ops::Index<usize> for Row {
    type Output = f64;
    fn index(&self, col: usize) -> &Self::Output {
        if col >= self.size { panic!("Index out-of-bounds") }
        &self.inner[col]
    }
}

impl PartialEq for Row {
    fn eq(&self, other: &Self) -> bool {
        const EPS: f64 = 0.00001;
        (0..self.size).all(|col| (self[col] - other[col]).abs() < EPS)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Matrix {
    inner: [Row; 4],
    pub size: usize
}

impl ops::Mul<Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Matrix {
        let mut m = self.empty();
        let size = self.size;
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
        if row >= self.size { panic!("Index out-of-bounds") }
        &self.inner[row]
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        
        self.size == other.size &&
        (0..self.size).all(|row| self[row] == other[row])
    }
}

impl Matrix {
    const EMPTY_ROW:  [f64; 4] = [0.0, 0.0, 0.0, 0.0];
    const EMPTY_ROW3: [f64; 3] = [0.0, 0.0, 0.0];
    const EMPTY_ROW2: [f64; 2] = [0.0, 0.0];

    fn new(row0: [f64; 4], row1: [f64; 4], row2 : [f64; 4], row3 : [f64; 4]) -> Matrix
    {
        Matrix { 
            inner: [ 
                Row { inner: row0, size: 4 }, 
                Row { inner: row1, size: 4 }, 
                Row { inner: row2, size: 4 }, 
                Row { inner: row3, size: 4 }], 
            size: 4}
    }

    fn new3(row0: [f64; 3], row1: [f64; 3], row2 : [f64; 3]) -> Matrix
    {
        Matrix { 
            inner: [ 
                Matrix::coerce_array3(row0), 
                Matrix::coerce_array3(row1), 
                Matrix::coerce_array3(row2), 
                Row { inner: Matrix::EMPTY_ROW, size: 3 }], 
            size: 3}
    }

    fn new2(row0: [f64; 2], row1: [f64; 2]) -> Matrix
    {
        Matrix { 
            inner: [ 
                Matrix::coerce_array2(row0), 
                Matrix::coerce_array2(row1), 
                Row { inner: Matrix::EMPTY_ROW, size: 2 }, 
                Row { inner: Matrix::EMPTY_ROW, size: 2 }], 
            size: 2 }
    }

    fn new_empty4() -> Matrix {
        Matrix::new(Matrix::EMPTY_ROW, Matrix::EMPTY_ROW, Matrix::EMPTY_ROW, Matrix::EMPTY_ROW)
    }

    fn new_empty3() -> Matrix {
        Matrix::new3(Matrix::EMPTY_ROW3, Matrix::EMPTY_ROW3, Matrix::EMPTY_ROW3)
    }

    fn new_empty2() -> Matrix {
        Matrix::new2(Matrix::EMPTY_ROW2, Matrix::EMPTY_ROW2)
    }

    fn coerce_array2(arr: [f64; 2]) -> Row {
        Row { inner: [arr[0], arr[1], 0.0, 0.0], size: 2 }
    }

    fn coerce_array3(arr: [f64; 3]) -> Row {
        Row { inner: [arr[0], arr[1], arr[2], 0.0], size: 3 }
    }

    fn empty(&self) -> Matrix {
        match self.size {
            2 => Matrix::new_empty2(),
            3 => Matrix::new_empty3(),
            4 => Matrix::new_empty4(),
            _ => { panic!("bad dimension") }
        }
    }

    fn identity_matrix() -> Matrix {
        Matrix { 
            inner: [
                Row { inner: [1.0, 0.0, 0.0, 0.0], size: 4 }, 
                Row { inner: [0.0, 1.0, 0.0, 0.0], size: 4 },
                Row { inner: [0.0, 0.0, 1.0, 0.0], size: 4 },
                Row { inner: [0.0, 0.0, 0.0, 1.0], size: 4 } ], 
            size: 4 }
    }

    fn set(&mut self, row: usize, col: usize, value: f64) {
        self.inner[row].inner[col] = value;
    }

    fn tuple(&self, row: usize) -> Tuple {
        let r = &self[row];
        Tuple::new(r[0], r[1], r[2], r[3])
    }

    fn transpose(&self) -> Matrix {
        let mut m = self.empty();
        let size = self.size;
        for row in 0..size {
            for col in 0..size {
                m.set(col, row, self[row][col]);
            }
        }
        m
    }

    fn determinant(&self) -> f64 {
        let size = self.size;
        match size {
            2 => self[0][0] * self[1][1] - self[0][1] * self[1][0],
            3...4 => {
                let r = &self[0].inner;
                let mut col = 0;
                r.iter().map(|c| { let v = c * self.cofactor(0, col); col += 1; v } ).sum()
            }
            _ => { panic!("Invalid matrix size, only 2x2, 3x3 and 4x4 supported") }
        }
    }

    fn submatrix(&self, row: usize, col: usize) -> Matrix {
        let size = self.size;
        let mut m = match size {
            4 => Matrix::new_empty3(),
            3 => Matrix::new_empty2(),
            _ => { panic!("Invalid matrix size, only 3x3 and 4x4 supported") }
        };
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

    fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        if (row + col) & 1 == 1 { -minor } else { minor }
    }

    fn inverse(&self) -> Option<Matrix> {
        let det = self.determinant();
        if det == 0.0 { return Option::None; }
        let size = self.size;
        let mut inverse = self.empty();
        for row in 0..size {
            for col in 0..size {
                inverse.set(col, row, self.cofactor(row, col) / det);
            }
        }
        Option::Some(inverse)
    }

    fn translation(x: f64, y: f64, z: f64) -> Matrix {
        Matrix::new_empty4()
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
        
        assert_eq!(a, a * Matrix::identity_matrix());
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
        assert_eq!(17.0, a.determinant());
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

    #[test]
    fn minor_of_3x3_matrix()
    {
        let a = Matrix::new3(
            [3.0, 5.0, 0.0],
            [2.0, -1.0, -7.0],
            [6.0, -1.0, 5.0]
        );
        let b = a.submatrix(1, 0);
        assert_eq!(25.0, b.determinant());
        assert_eq!(25.0, a.minor(1,0));
    }

    #[test]
    fn cofactor_of_3x3_matrix() {
        let a = Matrix::new3(
            [3.0, 5.0, 0.0],
            [2.0, -1.0, -7.0],
            [6.0, -1.0, 5.0]
        );
        assert_eq!(-12.0, a.minor(0, 0));
        assert_eq!(-12.0, a.cofactor(0, 0));
        assert_eq!(25.0, a.minor(1, 0));
        assert_eq!(-25.0, a.cofactor(1, 0));
    }

    #[test]
    fn determinant_of_3x3_matrix() {
        let a = Matrix::new3(
            [1.0, 2.0, 6.0],
            [-5.0, 8.0, -4.0],
            [2.0, 6.0, 4.0]
        );
        assert_eq!(56.0, a.cofactor(0, 0));
        assert_eq!(12.0, a.cofactor(0, 1));
        assert_eq!(-46.0, a.cofactor(0, 2));
        assert_eq!(-196.0, a.determinant());
    }

    #[test]
    fn determinant_of_4x4_matrix() {
        let a = Matrix::new(
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0]
        );
        assert_eq!(690.0, a.cofactor(0, 0));
        assert_eq!(447.0, a.cofactor(0, 1));
        assert_eq!(210.0, a.cofactor(0, 2));
        assert_eq!(51.0, a.cofactor(0, 3));
        assert_eq!(-4071.0, a.determinant());
    }

    #[test]
    fn matrix_is_invertible()
    {
        let a = Matrix::new(
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0]);
        assert_eq!(-2120.0, a.determinant());
        assert_ne!(Option::None, a.inverse());
    }

    #[test]
    fn matrix_is_not_invertible()
    {
        let a = Matrix::new(
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0]);
        assert_eq!(0.0, a.determinant());
        assert_eq!(Option::None, a.inverse());
    }

    #[test]
    fn inverse_of_matrix() {
        let a = Matrix::new(
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0]);
        let b = a.inverse().unwrap();
        assert_eq!(532.0, a.determinant());
        assert_eq!(-160.0, a.cofactor(2, 3));
        assert_eq!(-160.0 / 532.0, b[3][2]);
        assert_eq!(105.0, a.cofactor(3, 2));
        assert_eq!(105.0 / 532.0, b[2][3]);

        let expected = Matrix::new(
            [0.21805, 0.45113, 0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639]);
        assert_eq!(expected, b);
    }

    #[test]
    fn inverse_of_another_matrix() {
        let a = Matrix::new(
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0]);
        let b = a.inverse().unwrap();

        let expected = Matrix::new(
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692,  0.12308,  0.02564,  0.03077],
            [ 0.35897,  0.35897,  0.43590,  0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308]);
        assert_eq!(expected, b);
    }

    #[test]
    fn inverse_of_a_third_matrix() {
        let a = Matrix::new(
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0]);
        let b = a.inverse().unwrap();

        let expected = Matrix::new(
            [-0.04074, -0.07778,  0.14444, -0.22222],
            [-0.07778,  0.03333,  0.36667, -0.33333],
            [-0.02901, -0.14630, -0.10926,  0.12963],
            [ 0.17778,  0.06667, -0.26667,  0.33333]);
        assert_eq!(expected, b);
    }

    #[test]
    fn multiply_matrix_product_by_inverse() {
        let a = Matrix::new(
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0]);
        let b = Matrix::new(
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0]);
        let c = a * b;
        assert_eq!(a, c * b.inverse().unwrap());
    }

    #[test]
    fn multiply_by_translation_matrix()
    {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);
        let actual = transform * p;
        let expected = Tuple::point(2.0, 1.0, 7.0);
    }
}