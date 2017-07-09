use std::iter::repeat;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zeros_creates_zero_filled_vector_of_correct_size_and_length() {
        let a1 = zeros(vec![0]);
        assert_eq!(a1.shape, vec![0]);
        assert_eq!(a1.data, vec![]);

        let a2 = zeros(vec![1]);
        assert_eq!(a2.shape, vec![1]);
        assert_eq!(a2.data, vec![0]);

        let a3 = zeros(vec![10]);
        assert_eq!(a3.shape, vec![10]);
        assert_eq!(a3.data, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

        let a4 = zeros(vec![1, 2, 3]);
        assert_eq!(a4.shape, vec![1, 2, 3]);
        assert_eq!(a4.data, vec![0, 0, 0, 0, 0, 0]);

        let a5 = zeros(vec![3, 3]);
        assert_eq!(a5.shape, vec![3, 3]);
        assert_eq!(a5.data, vec![0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn full_test() {
        let a1 = full(vec![0], 1);
        assert_eq!(a1.shape, vec![0]);
        assert_eq!(a1.data, vec![]);

        let a2 = full(vec![1], -3);
        assert_eq!(a2.shape, vec![1]);
        assert_eq!(a2.data, vec![-3]);

        let a3 = full(vec![10], 5);
        assert_eq!(a3.shape, vec![10]);
        assert_eq!(a3.data, vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5]);

        let a4 = full(vec![1, 2, 3], 0);
        assert_eq!(a4.shape, vec![1, 2, 3]);
        assert_eq!(a4.data, vec![0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn index_test_1() {
        let array = QArray {
            shape: vec![1, 1, 1],
            data: vec![1],
        };

        assert_eq!(array.index(vec![0, 0, 0].as_slice()), Some(1));
    }

    #[test]
    fn index_test_2() {
        let array = QArray {
            shape: vec![2, 2, 1],
            data: vec![1, 2, 3, 4],
        };

        assert_eq!(array.index(vec![1, 0, 0].as_slice()), Some(2));
        assert_eq!(array.index(vec![0, 1, 0].as_slice()), Some(3));
    }

    #[test]
    fn index_test_3() {
        let array = QArray {
            shape: vec![2, 4, 3],
            data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                       22, 23, 24],
        };

        assert_eq!(array.index(vec![1, 2, 1].as_slice()), Some(14));
        assert_eq!(array.index(vec![0, 2, 2].as_slice()), Some(21));
    }

    #[test]
    fn test_1d_vec_to_qarray() {
        let v = vec![1, 2, 3];
        let array = v.to_qarray();

        assert_eq!(array.shape, vec![3]);
        assert_eq!(array.data, vec![1, 2, 3]);
    }

    #[test]
    fn test_2d_vec_to_qarray() {
        let v1 = vec![vec![1, 2], vec![3, 4]];
        let array1 = v1.to_qarray();

        assert_eq!(array1.shape, vec![2, 2]);
        assert_eq!(array1.data, vec![1, 2, 3, 4]);

        let v2 = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let array2 = v2.to_qarray();

        assert_eq!(array2.shape, vec![2, 3]);
        assert_eq!(array2.data, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_3d_vec_to_qarray() {
        let v = vec![vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6], vec![7, 8]]];
        let array = v.to_qarray();

        assert_eq!(array.shape, vec![2, 2, 2]);
        assert_eq!(array.data, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_identity() {
        let a1 = identity(1);
        assert_eq!(a1.shape, vec![1, 1]);
        assert_eq!(a1.data, vec![1]);

        let a2 = identity(2);
        assert_eq!(a2.shape, vec![2, 2]);
        assert_eq!(a2.data, vec![1, 0, 0, 1]);

        let a3 = identity(0);
        assert_eq!(a3.shape, vec![0, 0]);
        assert_eq!(a3.data, vec![]);
    }
}

/// Return a new QArray of the given shape, filled with zeroes.
///
/// ```
/// use quota::zeros;
/// let qarray = zeros(vec![1, 2, 3]);
/// assert_eq!(qarray.shape, vec![1, 2, 3]);
/// assert_eq!(qarray.data, vec![0, 0, 0, 0, 0, 0])
/// ```
pub fn zeros(shape: Vec<usize>) -> QArray {
    full(shape, 0)
}

/// Return a new QArray of the given shape, filled with the given fill value.
///
/// ```
/// use quota::full;
/// let qarray = full(vec![1, 2, 3], 2);
/// assert_eq!(qarray.shape, vec![1, 2, 3]);
/// assert_eq!(qarray.data, vec![2, 2, 2, 2, 2, 2])
/// ```
pub fn full(shape: Vec<usize>, fill_value: isize) -> QArray {
    let data = repeat(fill_value).take(shape.iter().product()).collect::<Vec<isize>>();
    QArray { shape, data }
}

/// Return a new square QArray of the given size, with ones on the main diagonal
///
/// ```
/// use quota::identity;
///
/// let qarray = identity(3);
/// assert_eq!(qarray.shape, vec![3, 3]);
/// assert_eq!(qarray.data, vec![1, 0, 0, 0, 1, 0, 0, 0, 1]);
/// ```
pub fn identity(num_rows: usize) -> QArray {
    let QArray { shape, mut data } = zeros(vec![num_rows, num_rows]);
    {
        for x in 0..num_rows {
            let offset = offset(shape.as_slice(), &[x, x]);
            data[offset] = 1;
        }
    }
    QArray { shape, data }
}

#[derive(Debug)]
pub struct QArray {
    pub shape: Vec<usize>,
    pub data: Vec<isize>,
}

impl QArray {
    /// Return the element at the given coordinates.
    /// If the coordinates don't specify a single element (e.g. 2D coordinates
    /// are given for a 3D array), or the coordinates are invalid,
    /// this function will return None.
    ///
    /// ```
    /// use quota::*;
    ///
    /// let qarray = zeros(vec![2, 2]);
    /// assert_eq!(qarray.index(vec![0, 0].as_slice()), Some(0));
    /// assert_eq!(qarray.index(vec![1, 1].as_slice()), Some(0));
    /// assert_eq!(qarray.index(vec![3, 3].as_slice()), None);
    /// assert_eq!(qarray.index(vec![1].as_slice()), None);
    /// ```
    pub fn index(&self, coordinates: &[usize]) -> Option<isize> {
        // Check that the coordinates are valid
        // The number of coordinates should match the shape of the array.
        // Each coordinate should be less than the length of the array
        // in that dimension.
        if coordinates.len() != self.shape.len() { return None }
        for (coord, dim) in coordinates.iter().zip(self.shape.as_slice()) {
            if *coord >= *dim { return None }
        }

        return Some(self.data[offset(self.shape.as_slice(), coordinates)])
    }
}

fn offset(shape: &[usize], coordinates: &[usize]) -> usize {
    coordinates
        .into_iter()
        .enumerate()
        .map(|(idx, coord)| {
                 shape
                     .iter()
                     .take(idx)
                     .fold(*coord, |acc, x| acc * x)
             })
        .fold(0, |acc, x| acc + x)
}

trait ToQArray {
    fn to_qarray(&self) -> QArray;
}

impl ToQArray for Vec<isize> {
    fn to_qarray(&self) -> QArray {
        QArray {
            shape: vec![self.len()],
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
        let shape = vec![l1, l2];

        let mut data = Vec::new();

        for row in self {
            for e in row {
                data.push(*e);
            }
        }

        QArray { shape, data }
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
        let shape = vec![l1, l2, l3];

        let mut data = Vec::new();

        for row in self {
            for col in row {
                for e in col {
                    data.push(*e)
                }
            }
        };

        QArray { shape, data }
    }
}
