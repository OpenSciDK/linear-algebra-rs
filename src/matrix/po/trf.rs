use crate::matrix::Matrix;
use crate::matrix::MatrixError;
use crate::number::c64;
use lapack::{dpotrf, zpotrf};
use std::error::Error;

impl Matrix {
    /// # Cholesky decomposition
    /// for positive definite f64 matrix
    ///
    /// https://en.wikipedia.org/wiki/Cholesky_decomposition
    ///
    /// `A = L * L^T`
    pub fn potrf(self) -> Result<Matrix, Box<dyn Error>> {
        let n = self.rows;
        if n != self.cols {
            return Err(Box::new(MatrixError::DimensionMismatch));
        }

        let mut info = 0;
        let mut slf = self;
        let n = n as i32;

        unsafe {
            dpotrf('U' as u8, n, &mut slf.elems, n, &mut info);
        }

        match info {
            0 => Ok(slf),
            _ => Err(Box::new(MatrixError::LapackRoutineError {
                routine: "dpotrf".to_owned(),
                info,
            })),
        }
    }
}

impl Matrix<c64> {
    /// # Cholesky decomposition
    /// for positive definite c64 matrix
    ///
    /// https://en.wikipedia.org/wiki/Cholesky_decomposition
    ///
    /// `A = L * L^*`
    pub fn potrf(self) -> Result<Matrix<c64>, Box<dyn Error>> {
        let n = self.rows;
        if n != self.cols {
            return Err(Box::new(MatrixError::DimensionMismatch));
        }

        let mut info = 0;
        let mut slf = self;
        let n = n as i32;

        unsafe {
            zpotrf('U' as u8, n, &mut slf.elems, n, &mut info);
        }

        match info {
            0 => Ok(slf),
            _ => Err(Box::new(MatrixError::LapackRoutineError {
                routine: "zpotrf".to_owned(),
                info,
            })),
        }
    }
}
