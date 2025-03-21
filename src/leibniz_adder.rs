use std::ops::Div;
use std::thread;

const LEIBNIZ_THRESHOLD: i32 = 200000;

pub fn get_leibniz_term(begin: i32, end: i32) -> f64 {
    if begin > end {
        return 0.0;
    }

    let term_amount = end - begin;

    if term_amount <= LEIBNIZ_THRESHOLD {
        return sequential_leibniz(begin, end);
    }

    let mid = (begin + end) / 2;

    let first_half = get_leibniz_term(begin, mid);
    let second_half = thread::spawn(move || get_leibniz_term(mid + 1, end));

    first_half + second_half.join().unwrap()
}

fn sequential_leibniz(begin: i32, end: i32) -> f64 {
    (begin..end + 1).map(|x| term(x)).sum::<f64>() * 4f64
}

fn term(i: i32) -> f64 {
    (-1f64).powi(i) / (2 * i + 1) as f64
}
