fn main () {
    standard_deviation_with_iters();
    standard_deviation();
}


fn standard_deviation_with_iters() {
    // Example data as a vector.
    let data = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];

    // Step 1: Calculate the mean (average).
    let mean = data.iter().sum::<f64>() / data.len() as f64;

    // Step 2: Calculate squared differences and collect them in a vector.
    let squared_differences: Vec<_> = data.iter().map(|x| (x - mean).powi(2)).collect();

    // Step 3: Calculate the mean of squared differences.
    let variance = squared_differences.iter().sum::<f64>() / squared_differences.len() as f64;

    // Step 4: Calculate the standard deviation (square root of the variance).
    let standard_deviation = variance.sqrt();

    println!("Mean: {:.2}", mean);
    println!("Variance: {:.2}", variance);
    println!("Standard Deviation: {:.2}", standard_deviation);
}


fn standard_deviation() {
    // Example data as a vector.
    let data = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];

    // Step 1: Calculate the mean (average).
    let mut sum = 0.0;
    for &x in &data {
        sum += x;
    }
    let mean = sum / data.len() as f64;

    // Step 2: Calculate squared differences and collect them in a vector.
    let mut squared_differences: Vec<f64> = Vec::new();
    for &x in &data {
        let diff = x - mean;
        squared_differences.push(diff * diff);
    }

    // Step 3: Calculate the mean of squared differences.
    let mut variance = 0.0;
    for &squared_diff in &squared_differences {
        variance += squared_diff;
    }
    variance /= squared_differences.len() as f64;

    // Step 4: Calculate the standard deviation (square root of the variance).
    let standard_deviation = variance.sqrt();

    println!("Mean: {:.2}", mean);
    println!("Variance: {:.2}", variance);
    println!("Standard Deviation: {:.2}", standard_deviation);
}
