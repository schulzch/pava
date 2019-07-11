use std::cmp::Ordering;

/// Regression result.
pub struct Regression {
    /// Number of points aggregated.
    pub num: Vec<usize>,
    /// Aggregated value.
    pub val: Vec<f64>,
}

/// Performs isotonic regression using the Pool-Adjacent-Violators-Algorithm (PAVA).
pub fn pava(values: Vec<f64>, ordering: Ordering) -> Regression {
    assert!(ordering != Ordering::Equal, "Requires a total order");
    let mut num = vec![0; values.len()];
    let mut val = vec![0.0; values.len()];
    num[0] = 1;
    val[0] = values[0];
    let mut j = 0;
    for i in 1..values.len() {
        j += 1;
        val[j] = values[i];
        num[j] = 1;
        while (j > 0) && val[j - 1].partial_cmp(&val[j]).unwrap() == ordering {
            val[j - 1] = ((num[j] as f64) * val[j] + (num[j - 1] as f64) * val[j - 1])
                / ((num[j] + num[j - 1]) as f64);
            num[j - 1] += num[j];
            j -= 1;
        }
    }
    num.truncate(j + 1);
    val.truncate(j + 1);
    Regression { num, val }
}
