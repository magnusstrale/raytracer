use core::ops;
//extern crate test;

type R4 = [f64;4];

type Matrix4 = [[f64; 4]; 4];
type Matrix3 = [[f64; 3]; 3];
type Matrix2 = [[f64; 2]; 2];

fn mul(lhs: Matrix4, rhs: Matrix4) -> Matrix4 {
    let mut m = [[0.0; 4]; 4];
    for row in 0..4 {
        for col in 0..4 {
            let mut acc: f64 = 0.0;
            for i in 0..4 {
                acc += lhs[row][i] * rhs[i][col];
            }
            m[row][col] = acc;
        }
    }
    m
    //Matrix4::from(m)
}

#[derive(Debug, Clone)]
struct Matrix {
    size: usize,
    m: Vec<Vec<f64>>
}

#[derive(Debug, Copy, Clone)]
struct Matrix4x4 {
    inner: [Row4; 4]
}

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

impl Matrix4x4 {
    fn new(row0: R4, row1: R4, row2 : R4, row3 : R4) -> Matrix4x4
    {
        Matrix4x4 { inner: [ Row4 { innerRow: row0}, Row4 { innerRow: row1}, Row4 { innerRow: row2}, Row4 { innerRow: row3}]}
    }
}

impl ops::Index<usize> for Matrix4x4 {
    type Output = Row4;
    fn index(&self, row: usize) -> &Self::Output {
        &self.inner[row]
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        const EPS: f64 = 0.00001;
        if self.size != other.size {
            return false;
        }
        for row in 0..self.size {
            for col in 0..self.size {
                if (self.m[row][col] - other.m[row][col]).abs() > EPS {
                    return false;
                }
            }
        }
        true
    }
}

impl From<Matrix> for Matrix4 {
    fn from(item: Matrix) -> Self {
        [[item.m[0][0], item.m[0][1], item.m[0][2], item.m[0][3]],
        [item.m[1][0], item.m[1][1], item.m[1][2], item.m[1][3]],
        [item.m[2][0], item.m[2][1], item.m[2][2], item.m[2][3]],
        [item.m[3][0], item.m[3][1], item.m[3][2], item.m[3][3]]]
    }
}

impl From<Matrix4> for Matrix {
    fn from(item: Matrix4) -> Self {
        Matrix::new4(&item[0], &item[1], &item[2], &item[3])
    }
}

impl From<Matrix3> for Matrix {
    fn from(item: Matrix3) -> Self {
        Matrix::new3(&item[0], &item[1], &item[2])
    }
}

impl From<Matrix2> for Matrix {
    fn from(item: Matrix2) -> Self {
        Matrix::new2(&item[0], &item[1])
    }
}

impl ops::Mul<Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Matrix {
        let mut m = Matrix::new(4);
        for row in 0..4 {
            for col in 0..4 {
                let mut acc: f64 = 0.0;
                for i in 0..4 {
                    acc += self.m[row][i] * rhs.m[i][col];
                }
                m.m[row][col] = acc;
            }
        }
        m
    }
}

impl Matrix {
    pub fn new(size: usize) -> Matrix {
        let row = vec![0.0; size];
        Matrix { size, m: vec![row; size] }
    }

    pub fn new2(row0: &[f64], row1: &[f64]) -> Matrix {
        let mut m = Matrix::new(2);
        m.m[0] = row0.to_vec();
        m.m[1] = row1.to_vec();
        m
    }

    pub fn new3(row0: &[f64], row1: &[f64], row2: &[f64]) -> Matrix {
        let mut m = Matrix::new(3);
        m.m[0] = row0.to_vec();
        m.m[1] = row1.to_vec();
        m.m[2] = row2.to_vec();
        m
    }

    pub fn new4(row0: &[f64], row1: &[f64], row2: &[f64], row3: &[f64]) -> Matrix {
        let mut m = Matrix::new(4);
        m.m[0] = row0.to_vec();
        m.m[1] = row1.to_vec();
        m.m[2] = row2.to_vec();
        m.m[3] = row3.to_vec();
        m
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    //use test::Bencher;


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
        let a: Matrix4 = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0]];
        let b: Matrix4 = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 1.0]];

        assert_ne!(a, b);
    }

    #[test]
    fn multiplying_two_matrices() {
        let a = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0]];
        let b: Matrix4 = [
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0]];
        let expected: Matrix4 = [
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0]];
        let a1 = Matrix::from(a);
        let b1 = Matrix::from(b);

        let result = a1 * b1;
        assert_eq!(expected, mul(a, b));
    }

    // #[bench]
    // fn array_based_matrix_multiplication(b: &mut Bencher) {
    //     let a: Matrix4 = [
    //         [1.0, 2.0, 3.0, 4.0],
    //         [5.0, 6.0, 7.0, 8.0],
    //         [9.0, 8.0, 7.0, 6.0],
    //         [5.0, 4.0, 3.0, 2.0]];
    //     let b: Matrix4 = [
    //         [-2.0, 1.0, 2.0, 3.0],
    //         [3.0, 2.0, 1.0, -1.0],
    //         [4.0, 3.0, 6.0, 5.0],
    //         [1.0, 2.0, 7.0, 8.0]];
    //     let c = mul(a, b);
    
    // }
}