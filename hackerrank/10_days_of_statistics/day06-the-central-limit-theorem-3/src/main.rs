fn main() {
    let n = 100.;
    let mu = 500.;
    let sigma = 80.;
    let z_score = 1.96;

    let mean_sample = (z_score * sigma) / (n as f64).sqrt();
    println!("{}", mu - mean_sample);
    println!("{}", mu + mean_sample);
}
