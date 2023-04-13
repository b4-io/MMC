use statrs::distribution::{ContinuousCDF, Normal};

pub fn ejercicio2() {
    let n = Normal::new(0.0, 1.0).unwrap();
    let delta: Vec<f64> = vec![0.001, 0.01, 0.05];
    let epsilon: f64 = 0.01;

    // Chebyshev worst case
    let mut chebyshev = vec![];
    for i in 0..delta.len() {
        chebyshev.push(1.0 / (4.0 * delta[i] * epsilon.powi(2)));
    }

    // LCT worst case
    let mut lct = vec![];
    for i in 0..delta.len() {
        lct.push((n.inverse_cdf(1.0 - delta[i] / 2.0) / (2.0 * epsilon)).powi(2));
    }

    // Hoeffding worst case
    let mut hoeffding = vec![];
    for i in 0..delta.len() {
        hoeffding.push(2.0 * (2.0 / delta[i]).ln() / (4.0 * epsilon.powi(2)));
    }

    // Output all worst cases
    println!("Epsilon: {}", epsilon);
    println!("Delta: {:?}", delta);
    println!("Chebyshev: {:?}", chebyshev);
    println!("LCT: {:?}", lct);
    println!("Hoeffding: {:?}", hoeffding);

}
