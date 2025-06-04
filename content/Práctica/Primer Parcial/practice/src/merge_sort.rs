use std::thread;

pub fn merge(first: &[i32], second: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let mut i = 0;
    let mut j = 0;

    // Merge until one of the inputs is exhausted
    while i < first.len() && j < second.len() {
        if first[i] <= second[j] {
            result.push(first[i]);
            i += 1
        } else {
            result.push(second[j]);
            j += 1
        }
    }
    // Copy the remaining items
    result.extend_from_slice(&first[i..]);
    result.extend_from_slice(&second[j..]);
    result
}

pub fn sort(array: &[i32]) -> Vec<i32> {
    let len = array.len();
    if len <= 1 {
        array.to_vec()
    } else {
        let x = sort(&array[..len / 2]); // <-- array slice
        let y = sort(&array[len / 2..]);
        merge(&x, &y)
    }
}

pub fn sort_parallel(array: &[i32]) -> Vec<i32> {
    let len = array.len();
    if len <= 1 {
        array.to_vec()
    } else {
        thread::scope(|s| {
            let x = sort(&array[..len / 2]);
            let y = s.spawn(move || sort(&array[len / 2..]));
            merge(&x, &(y.join().unwrap()))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    // 1. merge empty slices
    #[test]
    fn test_merge_empty() {
        let a: [i32; 0] = [];
        let b: [i32; 0] = [];
        assert_eq!(merge(&a, &b), Vec::<i32>::new());
    }

    // 2. merge one empty, one non‐empty
    #[test]
    fn test_merge_one_empty() {
        let a = [1, 3, 5];
        let b: [i32; 0] = [];
        assert_eq!(merge(&a, &b), vec![1, 3, 5]);
        assert_eq!(merge(&b, &a), vec![1, 3, 5]);
    }

    // 3. merge two interleaved lists
    #[test]
    fn test_merge_basic() {
        let a = [1, 4, 7];
        let b = [2, 3, 8, 9];
        assert_eq!(merge(&a, &b), vec![1, 2, 3, 4, 7, 8, 9]);
    }

    // 4. sort empty
    #[test]
    fn test_sort_empty() {
        let v: [i32; 0] = [];
        assert_eq!(sort(&v), Vec::<i32>::new());
        assert_eq!(sort_parallel(&v), Vec::<i32>::new());
    }

    // 5. sort single element
    #[test]
    fn test_sort_single() {
        let v = [42];
        assert_eq!(sort(&v), vec![42]);
        assert_eq!(sort_parallel(&v), vec![42]);
    }

    // 6. sort already sorted
    #[test]
    fn test_sort_sorted() {
        let v = [1, 2, 3, 4, 5];
        assert_eq!(sort(&v), vec![1, 2, 3, 4, 5]);
        assert_eq!(sort_parallel(&v), vec![1, 2, 3, 4, 5]);
    }

    // 7. sort reverse‐sorted
    #[test]
    fn test_sort_reverse() {
        let v = [5, 4, 3, 2, 1];
        let expected = vec![1, 2, 3, 4, 5];
        assert_eq!(sort(&v), expected);
        assert_eq!(sort_parallel(&v), expected);
    }

    // 8. sort with duplicates
    #[test]
    fn test_sort_duplicates() {
        let v = [3, 1, 2, 3, 1, 2];
        let expected = vec![1, 1, 2, 2, 3, 3];
        assert_eq!(sort(&v), expected);
        assert_eq!(sort_parallel(&v), expected);
    }

    // 9. sort a small random‐like array
    #[test]
    fn test_sort_random_small() {
        let v = [8, 3, 5, 1, 9, 2, 7, 4, 6, 0];
        let expected: Vec<i32> = (0..10).collect();
        assert_eq!(sort(&v), expected);
        assert_eq!(sort_parallel(&v), expected);
    }

    // 10. equivalence on a larger shuffled array
    #[test]
    fn test_sort_parallel_equivalence() {
        let mut v: Vec<i32> = (0..1000).map(|x| 1000 - x).collect();
        let v2 = v.clone();
        let seq = sort(&v);
        let par = sort_parallel(&v2);
        assert_eq!(seq, par);
    }

    // 11. performance: parallel must be faster than sequential
    #[test]
    fn test_sort_parallel_faster_than_sequential() {
        // large reverse‐sorted vector to maximize work
        let n = 50_000;
        let v: Vec<i32> = (0..n).map(|x| n - x).collect();
        let v2 = v.clone();

        let start_seq = Instant::now();
        let _ = sort(&v);
        let dur_seq = start_seq.elapsed();

        let start_par = Instant::now();
        let _ = sort_parallel(&v2);
        let dur_par = start_par.elapsed();

        assert!(
            dur_par < dur_seq,
            "Expected parallel ({:?}) < sequential ({:?})",
            dur_par,
            dur_seq
        );
    }
}
