extern crate rayon;

use rayon::prelude::*;

/// Integrate using the trapezoid rule
fn integrate<F>(f: F, a: f64, b: f64, n: u32) -> f64
where
    F: Fn(f64) -> f64,
{
    let h = (b - a) / n as f64;
    let mut result = 0.5 * f(a) + 0.5 * f(b);

    for i in 1..n {
        result += f(a + i as f64 * h);
    }
    result * h
}

/// Multithreaded Implementation
fn integrate_threaded<F>(f: F, a: f64, b: f64, n: u32) -> f64
where
    F: Fn(f64) -> f64 + Sync,
{
    let h = (b - a) / n as f64;
    let result: f64 = (1..n).into_par_iter().map(|i| f(a + i as f64 * h)).sum();
    h * result
}

fn main() {
    let f = |x: f64| x.powi(3);

    let result = integrate(f, 0.0, 1.0, 1000000);
    let result2 = integrate_threaded(f, 0.0, 1.0, 1000000);
    println!("{}, {}", result, result2);
    
}