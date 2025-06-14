use std::thread;
use std::thread::JoinHandle;

pub fn calculate_leibniz_thread(term: usize, parts: usize) -> f64{
    let parts_ranges = split_range(term, parts);
    let handles: Vec<JoinHandle<f64>> = parts_ranges.into_iter().map(
        |(start, end)| split_concurrent(start, end)
    ).collect();

    handles.into_iter().map(|handle| handle.join().unwrap()).sum::<f64>()
}

pub async fn calculate_leibniz_async(term:usize, parts: usize)-> f64{
    let parts_ranges = split_range(term, parts);
    let handles: Vec<tokio::task::JoinHandle<f64>> = parts_ranges.into_iter().map(
        |(start, end)| split_async(start, end)
    ).collect();
    let mut result:f64 = 0.0;
    for handle in handles{
        result += handle.await.unwrap()
    }
    result

}

fn split_concurrent(start: usize, end: usize)->JoinHandle<f64>{
    thread::spawn(move || leibniz_pi_partial(start, end))
}

fn split_async(start: usize, end: usize) -> tokio::task::JoinHandle<f64> {
    tokio::spawn(async move{leibniz_pi_partial(start, end)})
}



fn leibniz_pi_partial ( start : usize , count : usize ) -> f64 {
    ( start .. start + count )
        . map (| k | {
            let k = k as f64 ;
            ( -1.0f64 ) . powf ( k ) / (2.0 * k + 1.0)
        })
        . sum :: < f64 >()
        * 4.0
}

fn split_range(number: usize, m: usize) -> Vec<(usize, usize)> {
    let base = number / m;
    let rem  = number % m;
    let mut parts = Vec::with_capacity(m);
    let mut current = 0;

    for i in 0..=m {
        let size  = base + if i < rem { 1 } else { 0 };
        let start = current;
        let end   = current + size - 1;
        parts.push((start, end));
        current = end + 1;
    }

    parts
}