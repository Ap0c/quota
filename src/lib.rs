use std::iter::repeat;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zeros_creates_zero_filled_vector_of_correct_size_and_length() {
        let a1 = zeros(vec![0]);
        assert_eq!(a1.dimensions, vec![0]);
        assert_eq!(a1.data, vec![]);

        let a2 = zeros(vec![1]);
        assert_eq!(a2.dimensions, vec![1]);
        assert_eq!(a2.data, vec![0]);

        let a3 = zeros(vec![10]);
        assert_eq!(a3.dimensions, vec![10]);
        assert_eq!(a3.data, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

        let a4 = zeros(vec![1, 2, 3]);
        assert_eq!(a4.dimensions, vec![1, 2, 3]);
        assert_eq!(a4.data, vec![0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn index_test_1() {
        let array1 = QArray {
            dimensions: vec![1, 1, 1],
            data: vec![1],
        };

        assert_eq!(index(&array1, vec![0, 0, 0]), 0);
    }

    #[test]
    fn index_test_2() {
        let array2 = QArray {
            dimensions: vec![2, 2, 1],
            data: vec![1, 2, 3, 4],
        };

        assert_eq!(index(&array2, vec![1, 0, 0]), 1);
        assert_eq!(index(&array2, vec![0, 1, 0]), 2);
    }

    #[test]
    fn index_test_3() {
        let array3 = QArray {
            dimensions: vec![2, 4, 3],
            data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                       22, 23, 24],
        };

        assert_eq!(index(&array3, vec![1, 2, 1]), 13);
        assert_eq!(index(&array3, vec![0, 2, 2]), 20);
    }

    #[test]
    fn test_1d_vec_to_qarray() {
        let v = vec![1, 2, 3];
        let array = v.to_qarray();

        assert_eq!(array.dimensions, vec![3]);
        assert_eq!(array.data, vec![1, 2, 3]);
    }

    #[test]
    fn test_2d_vec_to_qarray() {
        let v1 = vec![vec![1, 2], vec![3, 4]];
        let array1 = v1.to_qarray();

        assert_eq!(array1.dimensions, vec![2, 2]);
        assert_eq!(array1.data, vec![1, 2, 3, 4]);

        let v2 = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let array2 = v2.to_qarray();

        assert_eq!(array2.dimensions, vec![2, 3]);
        assert_eq!(array2.data, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_3d_vec_to_qarray() {
        let v = vec![vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6], vec![7, 8]]];
        let array = v.to_qarray();

        assert_eq!(array.dimensions, vec![2, 2, 2]);
        assert_eq!(array.data, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }
}

/// Return a new QArray of the given dimensions, filled with zeroes.
///
/// ```
/// use quota::zeros;
/// let qarray = zeros(vec![1, 2, 3]);
/// assert_eq!(qarray.dimensions, vec![1, 2, 3]);
/// assert_eq!(qarray.data, vec![0, 0, 0, 0, 0, 0])
/// ```
pub fn zeros(dimensions: Vec<usize>) -> QArray {
    let data = repeat(0).take(dimensions.iter().sum()).collect::<Vec<isize>>();
    QArray { dimensions, data }
}

#[derive(Debug)]
pub struct QArray {
    pub dimensions: Vec<usize>,
    pub data: Vec<isize>,
}

pub fn index(array: &QArray, coordinates: Vec<usize>) -> usize {
    coordinates
        .into_iter()
        .enumerate()
        .map(|(idx, coord)| {
                 array
                     .dimensions
                     .iter()
                     .take(idx)
                     .fold(coord, |acc, x| acc * x)
             })
        .fold(0, |acc, x| acc + x)
}

trait ToQArray {
    fn to_qarray(&self) -> QArray;
}

impl ToQArray for Vec<isize> {
    fn to_qarray(&self) -> QArray {
        QArray {
            dimensions: vec![self.len()],
            data: self.clone(),
        }
    }
}

impl ToQArray for Vec<Vec<isize>> {
    fn to_qarray(&self) -> QArray {
        let l1 = self.len();
        let l2 = match self.first() {
            Some(fst) => fst.len(),
            None => 0,
        };
        let dimensions = vec![l1, l2];

        let mut data = Vec::new();

        for row in self {
            for e in row {
                data.push(*e);
            }
        }

        QArray { dimensions, data }
    }
}

impl ToQArray for Vec<Vec<Vec<isize>>> {
    fn to_qarray(&self) -> QArray {
        let l1 = self.len();
        let (l2, l3) = match self.first() {
            Some(fst) => match fst.first() {
                Some(ffst) => (fst.len(), ffst.len()),
                None => (fst.len(), 0)
            },
            None => (0, 0)
        };
        let dimensions = vec![l1, l2, l3];

        let mut data = Vec::new();

        for row in self {
            for col in row {
                for e in col {
                    data.push(*e)
                }
            }
        };

        QArray { dimensions, data }
    }
}
