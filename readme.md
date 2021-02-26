# matrix-multiply-benchmark-rs

## Dependencies

- rand;
- nalgebra.

## Run

```bash
$ cargo run --release -- -n 5000 -r 10
Compiling matrix-multiply-benchmark-rs v0.1.0 (/home/vehlwn/projects/rust/matrix-multiply-benchmark-rs)
Finished release [optimized] target(s) in 0.77s
Running `target/release/matrix-multiply-benchmark-rs -n 5000 -r 10`
[3.8633955, 3.8495688, 3.8353524, 3.9184046, 3.8977242, 3.8949547, 3.8988576, 3.9391487, 3.8924272, 3.8700514] s
min = 3.8353524 s
results.last = 1247.2684

$ cargo run --release -- -n 10000 -r 10
Finished release [optimized] target(s) in 0.01s
Running `target/release/matrix-multiply-benchmark-rs -n 10000 -r 10`
[30.449083, 30.169327, 30.856867, 30.91954, 30.677864, 30.9319, 30.87377, 30.833044, 31.034115, 30.903053] s
min = 30.169327 s
results.last = 2460.7427
```
