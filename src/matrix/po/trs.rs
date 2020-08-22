use crate::matrix::Matrix;
use crate::number::c64;
use lapack::{dpotrs, zpotrs};

impl Matrix {
    /// # Solve equation
    /// with matrix decomposed by potrf
    pub fn potrs(self, constants: Matrix) -> Result<Matrix, String> {
        let n = self.get_rows();
        if n != self.get_columns() {
            return Err("dimension mismatch".to_owned());
        }
        if n != constants.rows || constants.columns != 1 {
            return Err("dimension mismatch".to_owned());
        }

        let mut info = 0;

        let mut slf = self;
        let n = n as i32;
        let mut constants = constants;

        unsafe {
            dpotrs(
                'U' as u8,
                n,
                1,
                &mut slf.elements,
                n,
                &mut constants.elements,
                n,
                &mut info,
            );
        }

        match info {
            0 => Ok(slf),
            i => Err(i.to_string()),
        }
    }
}

impl Matrix<c64> {
    /// # Solve equation
    /// with matrix decomposed by potrf
    pub fn potrs(self, constants: Matrix<c64>) -> Result<Matrix<c64>, String> {
        let n = self.get_rows();
        if n != self.get_columns() {
            return Err("dimension mismatch".to_owned());
        }
        if n != constants.rows || constants.columns != 1 {
            return Err("dimension mismatch".to_owned());
        }

        let mut info = 0;

        let mut slf = self;
        let n = n as i32;
        let mut constants = constants;

        unsafe {
            zpotrs(
                'U' as u8,
                n,
                1,
                &mut slf.elements,
                n,
                &mut constants.elements,
                n,
                &mut info,
            );
        }

        match info {
            0 => Ok(slf),
            i => Err(i.to_string()),
        }
    }
}