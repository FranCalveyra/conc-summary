use std::thread;
use std::thread::JoinHandle;

fn sum_parallel(nums: Vec<i32>, m: usize) -> i32 {
    let mut parts = split_into_parts(nums, m);

    let first = parts.remove(0);
    let result: i32 = first.iter().sum();

    let handles: Vec<JoinHandle<i32>> = parts
        .into_iter()
        .map(|chunk| thread::spawn(move || chunk.iter().sum()))
        .collect();

    result
        + handles
            .into_iter()
            .map(|h| h.join().unwrap_or(0))
            .sum::<i32>()
}

fn split_into_parts<T>(mut vec: Vec<T>, m: usize) -> Vec<Vec<T>> {
    assert!(m > 0, "M should be ≥ 1");
    let n = vec.len();
    let base = n / m;
    let mut rem = n % m;

    let mut parts = Vec::with_capacity(m);
    for _ in 0..m {
        let this_size = base
            + if rem > 0 {
                rem -= 1;
                1
            } else {
                0
            };
        parts.push(vec.drain(0..this_size).collect());
    }
    parts
}

#[cfg(test)]
mod tests {
    use super::sum_parallel;
    #[test]
    fn empty_vector_should_return_zero() {
        let vec: Vec<i32> = vec![];
        assert_eq!(0, sum_parallel(vec, 1))
    }

    #[test]
    fn vector_with_one_element_should_return_that_element() {
        for x in 0..100 {
            let vec: Vec<i32> = vec![x];
            assert_eq!(x, sum_parallel(vec, 1))
        }
    }

    #[test]
    fn should_add_all_numbers_in_vector() {
        let vec: Vec<i32> = (1..100).collect();
        for m in 1..10 {
            assert_eq!(4950, sum_parallel(vec.clone(), m))
        }
    }

    #[test]
    fn should_get_the_same_result_as_iterative() {
        let vec: Vec<i32> = (1..1000).collect();
        let sequential_sum: i32 = vec.iter().sum();
        let conc_sum = sum_parallel(vec, 8);
        assert_eq!(sequential_sum, conc_sum)
    }

    #[test]
    #[should_panic]
    fn should_panic_when_m_is_zero() {
        let vec: Vec<i32> = (1..1000).collect();
        sum_parallel(vec, 0);
    }
}
