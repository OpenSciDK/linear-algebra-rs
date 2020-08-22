use crate::matrix::Matrix;
use crate::number::c64;
use lapack::{dgetrf, zgetrf};

impl Matrix {
    /// # LU decomposition
    /// for f64
    pub fn getrf(self) -> Result<(Matrix, Vec<i32>), String> {
        let m = self.rows;
        let n = self.columns;
        let mut ipiv = vec![0; m.min(n)];
        let mut info = 0;

        let mut slf = self;
        let m = m as i32;
        let n = n as i32;

        unsafe {
            dgetrf(n, m, &mut slf.elements, n, &mut ipiv, &mut info);
        }

        match info {
            0 => Ok((slf, ipiv)),
            i => Err(i.to_string()),
        }
    }
}

impl Matrix<c64> {
    /// # LU decomposition
    /// for c64
    pub fn getrf(self) -> Result<(Matrix<c64>, Vec<i32>), String> {
        let m = self.rows;
        let n = self.columns;
        let mut ipiv = vec![0; m.min(n)];
        let mut info = 0;

        let mut slf = self;
        let m = m as i32;
        let n = n as i32;

        unsafe {
            zgetrf(n, m, &mut slf.elements, n, &mut ipiv, &mut info);
        }

        match info {
            0 => Ok((slf, ipiv)),
            i => Err(i.to_string()),
        }
    }
}