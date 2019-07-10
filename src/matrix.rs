use core::ops;
//extern crate test;

type Matrix3 = [[f64; 3]; 3];
type Matrix2 = [[f64; 2]; 2];

type R4 = [f64;4];

#[derive(Debug, Copy, Clone)]
struct Row4 {
    innerRow: R4
}

impl ops::Index<usize> for Row4 {
    type Output = f64;
    fn index(&self, col: usize) -> &Self::Output {
        &self.innerRow[col]
    }
}

impl PartialEq for Row4 {
    fn eq(&self, other: &Self) -> bool {
        const EPS: f64 = 0.00001;
        for col in 0..4 {
            if (self[col] - other[col]).abs() > EPS {
                return false;
            }
        }
        true
    }
}

#[derive(Debug, Copy, Clone)]
struct Matrix4x4 {
    inner: [Row4; 4]
}

impl ops::Mul<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, rhs: Matrix4x4) -> Matrix4x4 {
        let mut m = Matrix4x4::new_empty();
        for row in 0..4 {
            for col in 0..4 {
                let mut acc: f64 = 0.0;
                for i in 0..4 {
                    acc += self[row][i] * rhs[i][col];
                }
                m.set(row, col, acc);
            }
        }
        m
    }
}

impl ops::Index<usize> for Matrix4x4 {
    type Output = Row4;
    fn index(&self, row: usize) -> &Self::Output {
        &self.inner[row]
    }
}

impl PartialEq for Matrix4x4 {
    fn eq(&self, other: &Self) -> bool {
        for row in 0..4 {
            if self[row] != other[row] {
                return false;
            }
        }
        true
    }
}

impl Matrix4x4 {
    const EMPTY_ROW: R4 = [0.0, 0.0, 0.0, 0.0];

    fn new(row0: R4, row1: R4, row2 : R4, row3 : R4) -> Matrix4x4
    {
        Matrix4x4 { inner: [ Row4 { innerRow: row0}, Row4 { innerRow: row1}, Row4 { innerRow: row2}, Row4 { innerRow: row3}]}
    }

    fn new_empty() -> Matrix4x4 {
        Matrix4x4::new(Matrix4x4::EMPTY_ROW, Matrix4x4::EMPTY_ROW, Matrix4x4::EMPTY_ROW, Matrix4x4::EMPTY_ROW)
    }

    fn set(&mut self, row: usize, col: usize, value: f64) {
        self.inner[row].innerRow[col] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_4x4_matrix()
    {
        let m = Matrix4x4::new(
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
    fn construct_2x2_matrix()
    {
        let m: Matrix2 = [
            [-3.0, 5.0],
            [1.0, -2.0]];
        assert_eq!(-3.0, m[0][0]);
        assert_eq!(5.0, m[0][1]);
        assert_eq!(1.0, m[1][0]);
        assert_eq!(-2.0, m[1][1]);
    }

    #[test]
    fn construct_3x3_matrix()
    {
        let m: Matrix3 = [
            [-3.0, 5.0, 0.0],
            [1.0, -2.0, -7.0],
            [0.0, 1.0, 1.0]];
        assert_eq!(-3.0, m[0][0]);
        assert_eq!(-2.0, m[1][1]);
        assert_eq!(1.0, m[2][2]);
    }

    #[test]
    fn matrix_equality_identical_matrices() {
        let a = Matrix4x4::new(
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0]);
        let b = Matrix4x4::new(
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0]);

        assert_eq!(a, b);
    }

    #[test]
    fn matrix_equality_different_matrices() {
        let a = Matrix4x4::new(
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0]);
        let b = Matrix4x4::new(
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 1.0]);

        assert_ne!(a, b);
    }

    #[test]
    fn multiplying_two_matrices() {
        let a = Matrix4x4::new (
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0]);
        let b = Matrix4x4::new(
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0]);
        let expected = Matrix4x4::new(
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0]);

        let result = a * b;
        assert_eq!(expected, result);
    }
}