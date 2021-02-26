mod config;

fn main() {
    let config = config::parse_command_line();
    let mut results = Vec::new();
    let mut result_secs = Vec::new();
    results.reserve(config.r);
    result_secs.reserve(config.r);
    let mut a = nalgebra::DMatrix::<f32>::new_random(config.n, config.n);
    let b = nalgebra::DMatrix::<f32>::new_random(config.n, config.n);
    for _ in 0..config.r {
        let t1 = std::time::Instant::now();
        let c = a.clone() * &b;
        let t2 = std::time::Instant::now();
        result_secs.push((t2 - t1).as_secs_f32());
        results.push(c[(config.n - 1, config.n - 1)]);
        a[(config.n - 1, config.n - 1)] = rand::random();
    }
    println!("{:?} s", result_secs);
    println!(
        "min = {} s",
        result_secs
            .iter()
            .cloned()
            .fold(f32::INFINITY, |a, b| a.min(b))
    );
    println!("results.last = {}", results.last().unwrap());
}
