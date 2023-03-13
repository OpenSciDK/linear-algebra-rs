use crate::sparse::RankIndex;
use crate::tensor::Tensor;
use crate::{generate_rank_combinations, RankCombinationId, TensorError};
use crate::{sparse::SparseTensor, Number};
use rand::prelude::*;
use std::collections::HashMap;

pub trait DotProduct<T>
where
    T: Number,
{
    fn dot_product(
        self,
        rank_combinations: &[HashMap<RankIndex, RankCombinationId>],
    ) -> SparseTensor<T>;
}

impl<'a, I, T> DotProduct<T> for I
where
    I: Iterator<Item = &'a SparseTensor<T>>,
    T: Number + 'a,
{
    fn dot_product(
        self,
        rank_combinations: &[HashMap<RankIndex, RankCombinationId>],
    ) -> SparseTensor<T> {
        let terms = self.collect::<Vec<_>>();
        let max_rank = terms.iter().map(|t| t.rank()).max().unwrap();
        let mut new_sizes = vec![1; max_rank];
        println!("new_sizes0: {:?}", new_sizes);

        for (i, t) in terms.iter().enumerate() {
            for (j, &dim) in t.sizes.iter().enumerate() {
                println!("t.sizes:{:?}", t.sizes);
                println!("i: {:?}, j: {:?}, dim: {:?}", i, j, dim);
                println!(
                    "rank_combinations[i].get[&j]: {:?}",
                    rank_combinations[i].get(&j)
                );
                if rank_combinations[i].get(&j).is_none() && dim > 1 {
                    if new_sizes[j] == 1 {
                        new_sizes[j] = dim;
                    } else {
                        panic!("The tensor whose a rank that is not aggregated and has a dimension greater than 1 can't be included.")
                    }
                }
            }
        }
        println!("new_sizes1: {:?}", new_sizes);

        let mut result = SparseTensor::<T>::new(new_sizes.clone());
        //rank_combination = [2, 1]のとき
        //kの場所が変わる
        for i in 0..new_sizes[0] {
            for j in 0..new_sizes[1] {
                for k in 0..max_rank {
                    let mut first_term_index: [&[usize]; 1] = [&[0, 0]];
                    if todo!() {
                        //rank_combination[0] = 1のとき
                        first_term_index[0][0] = i;
                    } else {
                        //rank_combination[0] = 2のとき
                        first_term_index[0][1] = i;
                    }

                    // println!("i: {:?}, j: {:?}, k: {:?}", i, j, k);
                    result[&[i, j]] += terms[0][&[i, k]] * terms[1][&[k, j]];
                    // println!("{:?}:, {:?}", [&[i, j]], result[&[i, j]]);
                }
            }
        }

        fn setIndex(rank_combinations: &[HashMap<RankIndex, RankCombinationId>]) {
            let first_term_index = rank_combinations[0].get(&0).unwrap();
            println!("first_term_index: {:?}", first_term_index);
            let second_term_index = rank_combinations[1].get(&0).unwrap();
            println!("second_term_index: {:?}", second_term_index);
        }

        result
    }
}

impl<T> SparseTensor<T>
where
    T: Number,
{
    pub fn dot(&self, rhs: &Self, rank_pairs: &[[RankIndex; 2]]) -> Self {
        let rank_combinations = generate_rank_combinations(rank_pairs);

        vec![self, rhs].into_iter().dot_product(&rank_combinations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sparse::SparseTensor;
    use crate::tensor::Tensor;
    use crate::Number;

    #[test]
    fn test_dot_product() {
        let mut a = SparseTensor::<f64>::new(vec![2, 2]);
        a[&[0, 0]] = 1.0;
        a[&[0, 1]] = 2.0;
        a[&[1, 0]] = 3.0;
        a[&[1, 1]] = 4.0;

        let mut b = SparseTensor::<f64>::new(vec![2, 2]);
        b[&[0, 0]] = 2.0;
        b[&[0, 1]] = 4.0;
        b[&[1, 0]] = 6.0;
        b[&[1, 1]] = 8.0;

        let mut c = SparseTensor::<f64>::new(vec![2, 2]);
        c[&[0, 0]] = 1.0;
        c[&[0, 1]] = 2.0;
        c[&[1, 0]] = 3.0;
        c[&[1, 1]] = 4.0;

        let mut d = SparseTensor::<f64>::new(vec![2, 2]);
        d[&[0, 0]] = 1.0;
        d[&[0, 1]] = 2.0;
        d[&[1, 0]] = 3.0;
        d[&[1, 1]] = 4.0;

        let rank_pairs = [[0, 2], [2, 1]];
        let rank_combinations = generate_rank_combinations(&rank_pairs);
        println!("rank_combinations:{:?}", rank_combinations);
        println!("rank:{:?}", rank_combinations[0].get(&0));

        let result = vec![&a, &b].into_iter().dot_product(&rank_combinations);

        // let result = vec![&a, &b, &c, &d]
        //     .into_iter()
        //     .dot_product(&rank_combinations);

        println!("result:{:?}", result);

        // let expected = SparseTensor::<f64>::from_vec(
        //     vec![
        //         1., 2., 3., 4., 2., 4., 6., 8., 3., 6., 9., 12., 4., 8., 12., 16.,
        //     ],
        //     vec![2, 2, 2, 2],
        // );

        // assert_eq!(result, expected);
    }
}
