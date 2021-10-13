fn mean_var(arr: &[f64]) -> (f64, f64) {
    let dim = arr.len();
    if dim == 0 {
        return (0.0, 0.0);
    }
    let mut sum_orig = 0.0;
    let mut sum_square = 0.0;
    for ele in arr {
        sum_orig += ele;
        sum_square += ele.powi(2);
    }
    let mean = sum_orig / dim as f64;
    let variance = sum_square / dim as f64 - mean.powi(2);
    (mean, variance)
}

fn main() {
    let a = vec![1.0, 2.1, 2.3, 3.4];
    println!("{:?}", mean_var(&a));
}
