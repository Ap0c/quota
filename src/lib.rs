use std::iter::repeat;

#[cfg(test)]
mod tests {
    #[test]
    fn zeros_creates_zero_filled_vector_of_correct_size_and_length() {
        use super::zeros;
        assert_eq!(zeros(0), Vec::new());
        assert_eq!(zeros(1), vec![0]);
        assert_eq!(zeros(10), vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
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
