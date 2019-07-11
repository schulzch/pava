use std::cmp::Ordering;

/// Regression result.
pub struct Regression {
    /// Aggregated weights, i.e., the number of points if all weights were one.
    pub weights: Vec<f64>,
    /// Aggregated values.
    pub values: Vec<f64>,
}

/// Performs isotonic regression using the Pool-Adjacent-Violators-Algorithm (PAVA).
///
/// The algorithm minimizes $$\sum_{i=1}^{n} w_{i}(\hat{x}_i - x_i)^2$$ so that
/// $$\hat{x}_i \leq \hat{x}_{i+1}$$, or respectively $$\hat{x}_i \geq \hat{x}_{i+1}$$.
pub fn pava(values: &[f64], weights: &[f64], ordering: Ordering) -> Regression {
    assert!(ordering != Ordering::Equal, "Requires a total order");
    let mut w = vec![0.0; values.len()];
    let mut x = vec![0.0; values.len()];
    x[0] = values[0];
    w[0] = weights[0];
    let mut j = 0;
    let mut s = vec![0; values.len()];
    for i in 1..values.len() {
        j += 1;
        x[j] = values[i];
        w[j] = weights[i];
        while (j > 0) && x[j - 1].partial_cmp(&x[j]).unwrap() == ordering {
            x[j - 1] = (w[j] * x[j] + w[j - 1] * x[j - 1]) / (w[j] + w[j - 1]);
            w[j - 1] += w[j];
            j -= 1;
        }
        s[j + 1] = i + 1;
    }
    w.truncate(j + 1);
    x.truncate(j + 1);
    s.truncate(j + 2);
    // Map from new points to old points.
    let mut ww = vec![0.0; values.len()];
    let mut xx = vec![0.0; values.len()];
    for k in 0..(j + 1) {
        for i in s[k]..s[k + 1] {
            ww[i] = w[k];
            xx[i] = x[k];
        }
    }
    Regression {
        weights: ww,
        values: xx,
    }
}
