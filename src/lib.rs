use std::ops::Not;

/// Isotonic regression result.
pub struct Regression {
    /// Aggregated weights, i.e., the number of points if all weights were one.
    pub weights: Vec<f64>,
    /// Aggregated values.
    pub values: Vec<f64>,
}

/// An ordering for `Regression`.
#[derive(Clone, Copy, Debug)]
pub enum Ordering {
    Increasing,
    Decreasing,
}

impl Not for Ordering {
    type Output = Ordering;

    fn not(self) -> Self::Output {
        match self {
            Ordering::Increasing => Ordering::Decreasing,
            Ordering::Decreasing => Ordering::Increasing,
        }
    }
}

impl Into<std::cmp::Ordering> for Ordering {
    fn into(self) -> std::cmp::Ordering {
        match self {
            Ordering::Increasing => std::cmp::Ordering::Greater,
            Ordering::Decreasing => std::cmp::Ordering::Less,
        }
    }
}

/// Isotonic regression using the Pool-Adjacent-Violators algorithm (PAVA).
///
/// <script type="text/javascript" async src="https://cdnjs.cloudflare.com/ajax/libs/mathjax/2.7.5/MathJax.js?config=TeX-AMS_CHTML"></script>
///
/// The algorithm minimizes $$ \sum_{i}w_{i}(\hat{x}_i - x_i)^2 $$ while
/// ensuring the specified order. The result is only vaild if \( w_{i} \ge 0 \).
impl Regression {
    /// Isotonic regression to establish an axis-aligned increasing order
    /// ( \( \hat{x}_i \leq \hat{x}_{i+1} \) ), or decreasing order
    /// ( \( \hat{x}_i \geq \hat{x}_{i+1} \) ).
    pub fn new(values: &[f64], weights: &[f64], ordering: Ordering) -> Self {
        assert!(
            values.len() == weights.len(),
            "Values and weights must be equal-sized"
        );
        debug_assert!(
            weights.iter().all(|&x| x >= 0.0),
            "Weights must be positive"
        );
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
            while (j > 0) && x[j - 1].partial_cmp(&x[j]).unwrap() == ordering.into() {
                x[j - 1] = (w[j] * x[j] + w[j - 1] * x[j - 1]) / (w[j] + w[j - 1]);
                w[j - 1] += w[j];
                j -= 1;
            }
            s[j + 1] = i + 1;
        }
        //XXX: code is inefficient (wasting memory)
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

    /// Isotonic regression to establish a radial-center-outward (increasing) order, or
    /// a radial-center-inward (decreasing) order.
    pub fn new_radial(
        values: &[f64],
        weights: &[f64],
        center_index: usize,
        ordering: Ordering,
    ) -> Self {
        let mut a = Self::new(&values[..center_index], &weights[..center_index], !ordering);
        let mut b = Self::new(&values[center_index..], &weights[center_index..], ordering);
        a.weights.append(&mut b.weights);
        a.values.append(&mut b.values);
        a
    }
}
