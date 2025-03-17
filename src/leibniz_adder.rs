pub fn get_term(i: i32) -> f64 {
    (0..i + 1).map(|x| term(x)).sum::<f64>() * 4f64
}

fn term(i: i32) -> f64 {
    (-1f64).powi(i) / (2 * i + 1) as f64
}
