use std::f64;

// Define a function to compute the maximum likelihood estimate using the Newton-Raphson method
fn max_likelihood_estimate_newton(data: &[f64], tol: f64, max_iter: usize) -> f64 {
    let n = data.len() as f64;

    // Define the log likelihood function
    let log_likelihood = |mu: f64| -> f64 {
        let sum = data.iter().map(|x| (x - mu).ln()).sum::<f64>();
        n * mu.ln() - n * mu - sum
    };

    // Define the derivative of the log likelihood function
    let log_likelihood_deriv = |mu: f64| -> f64 {
        let sum = data.iter().map(|x| 1.0 / (x - mu)).sum::<f64>();
        n / mu - sum
    };

    // Initialize the estimate to the sample mean
    let mut mu = data.iter().sum::<f64>() / n;

    // Iterate until convergence or maximum number of iterations is reached
    let mut iter = 0;
    let mut delta = f64::INFINITY;

    while delta > tol && iter < max_iter {
        let prev_mu = mu;
        let f = log_likelihood_deriv(prev_mu);
        let f_prime = -1.0 / (prev_mu.powi(2)) * data.iter().map(|x| 1.0 / (x - prev_mu)).sum::<f64>();
        mu = prev_mu - f / f_prime;
        delta = (mu - prev_mu).abs();
        iter += 1;
    }

    mu
}

fn main() {
    // Example usage
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let mle = max_likelihood_estimate_newton(&data, 1e-6, 100);
    println!("Maximum likelihood estimate: {}", mle);
}
