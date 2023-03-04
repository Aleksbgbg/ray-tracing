# Ray Tracing

[Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) in Rust.

## Dev Workflow

### Run

`cargo run > image.ppm`

For reasonable performance:

`cargo run --release > image.ppm`

## Benchmarks

System:

- CPU: AMD Ryzen 7 3800X
- OS: Windows 10.0.19044

Average time to render in release:

- Single-threaded: 2754.54ms
- Multi-threaded (16 threads): 299.48ms
