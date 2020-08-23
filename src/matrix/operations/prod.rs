use crate::{matrix::Matrix, number::Number};
use rayon::prelude::*;

impl<T> Matrix<T>
where
    T: Number,
{
    /// # Linear product
    pub fn linear_prod(&self, rhs: &Matrix<T>) -> T {
        if !self.is_same_size(rhs) {
            panic!("dimension mismatch")
        } else {
            self.elements
                .par_iter()
                .zip(rhs.elements.par_iter())
                .map(|(&s, &r)| s * r)
                .sum()
        }
    }

    /// # Hamadard product
    pub fn hadamard_prod(self, rhs: &Matrix<T>) -> Matrix<T> {
        if !self.is_same_size(rhs) {
            panic!("dimension mismatch")
        }
        let mut slf = self;

        slf.elements
            .par_iter_mut()
            .zip(rhs.elements.par_iter())
            .map(|(s, &r)| {
                *s *= r;
            })
            .collect::<Vec<_>>();

        slf
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        let a = Matrix::<f64>::identity(2);
        let b = Matrix::<f64>::identity(2);
        let c = a.hadamard_prod(&b);

        assert_eq!(b[0][0], c[0][0])
    }
}
