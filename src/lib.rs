use std::iter::repeat;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zeros_creates_zero_filled_vector_of_correct_size_and_length() {
        assert_eq!(zeros(0), Vec::new());
        assert_eq!(zeros(1), vec![0]);
        assert_eq!(zeros(10), vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn index_test_1() {
        let array1 = QArray {
            dimensions: vec![1, 1, 1],
            data: vec![1]
        };

        assert_eq!(index(&array1, vec![0, 0, 0]), 0);
    }

    #[test]
    fn index_test_2() {
        let array2 = QArray {
            dimensions: vec![2, 2, 1],
            data: vec![1, 2, 3, 4]
        };

        assert_eq!(index(&array2, vec![1, 0, 0]), 1);
        assert_eq!(index(&array2, vec![0, 1, 0]), 2);
    }

    #[test]
    fn index_test_3() {
        let array3 = QArray {
            dimensions: vec![2, 4, 3],
            data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
                       11, 12, 13, 14, 15, 16, 17,
                       18, 19, 20, 21, 22, 23, 24]
        };

        assert_eq!(index(&array3, vec![1, 2, 1]), 13);
        assert_eq!(index(&array3, vec![0, 2, 2]), 20);
    }
}

/// Return a new vector of the given length, filled with zeroes.
/// TODO: make this work for N dimensional vectors
///
/// ```
/// use quota::zeros;
/// assert_eq!(zeros(4), vec![0, 0, 0, 0])
/// ```
pub fn zeros(n: usize) -> Vec<isize> {
    repeat(0).take(n).collect::<Vec<isize>>()
}

pub struct QArray {
    dimensions: Vec<usize>,
    data: Vec<isize>
}

pub fn index(array: &QArray, coordinates: Vec<usize>) -> usize {
    coordinates.into_iter().enumerate().map(|(idx, coord)| {
        array.dimensions.iter().take(idx).fold(coord, |acc, x| acc * x )
    }).fold(0, |acc, x| acc + x)
}
