pub fn get_term(i: i32) -> f64 {
    let mut total = 0.0;
    for x in 0..i+1 {
        total += term(x) * 4f64;
    }
    total
}

fn term(i: i32) -> f64 {
    (-1f64).powi(i) / (2 * i + 1) as f64
}
