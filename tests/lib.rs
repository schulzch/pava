extern crate pava;
extern crate rand;
extern crate svg;

use rand::{Rng, SeedableRng, StdRng};
use std::f64;
use svg::node::element::{Circle, Line};
use svg::Document;

fn dump_svg(name: &str, values_in: &[f64], values_out: &[f64]) {
    let (min_x, min_y, max_x, max_y) = values_in.iter().enumerate().fold(
        (f64::MAX, f64::MAX, f64::MIN, f64::MIN),
        |extrema, (x, &y)| {
            (
                extrema.0.min(x as f64),
                extrema.1.min(y),
                extrema.2.max(x as f64),
                extrema.3.max(y),
            )
        },
    );
    let (min_x, min_y, max_x, max_y) =
        values_out
            .iter()
            .enumerate()
            .fold((min_x, min_y, max_x, max_y), |extrema, (x, &y)| {
                (
                    extrema.0.min(x as f64),
                    extrema.1.min(y),
                    extrema.2.max(x as f64),
                    extrema.3.max(y),
                )
            });
    let margin = 5.0;
    let mut doc = Document::new().set(
        "viewBox",
        (
            min_x - margin,
            min_y - margin,
            (max_x - min_x) + 2.0 * margin,
            (max_y - min_y) + 2.0 * margin,
        ),
    );
    for (x, &y) in values_out.iter().enumerate() {
        let line = Line::new()
            .set("stroke", "#fb8072")
            .set("stroke-width", "0.1")
            .set("x1", x as f64)
            .set("y1", max_y - y as f64)
            .set("x2", x as f64)
            .set("y2", max_y - values_in[x] as f64);
        doc = doc.add(line);
        let circle = Circle::new()
            .set("fill", "#fb8072")
            .set("r", "0.4")
            .set("cx", x as f64)
            .set("cy", max_y - y);
        doc = doc.add(circle);
    }
    for (x, &y) in values_in.iter().enumerate() {
        let circle = Circle::new()
            .set("fill", "black")
            .set("r", 0.2)
            .set("cx", x as f64)
            .set("cy", max_y - y);
        doc = doc.add(circle);
    }
    svg::save(format!("target/_{}.svg", name), &doc).expect("Writing SVG failed");
}

fn noisy_line<F>(f: F) -> (Vec<f64>, Vec<f64>)
where
    F: Fn(usize, usize) -> usize,
{
    let mut seed = [0u8; 32];
    seed.copy_from_slice((0..32).map(|i| i + 1).collect::<Vec<u8>>().as_slice());
    let mut random: StdRng = SeedableRng::from_seed(seed);
    let values: Vec<f64> = (0..100)
        .map(|i| f(i, 100) as f64 * 0.5 + random.gen_range(-10.0, 10.0))
        .collect();
    let weights = vec![1.0; values.len()];
    (values, weights)
}

#[test]
fn increasing() {
    let (values, weights) = noisy_line(|i, _| i);
    let res = pava::Regression::new(&values, &weights, pava::Ordering::Increasing);
    dump_svg("increasing", &values, &res.values);
}

#[test]
fn decreasing() {
    let (values, weights) = noisy_line(|i, n| n - i);
    let res = pava::Regression::new(&values, &weights, pava::Ordering::Decreasing);
    dump_svg("decreasing", &values, &res.values);
}

#[test]
fn radial_decreasing() {
    let (mut values_a, mut weights_a) = noisy_line(|i, _| i);
    let (mut values_b, mut weights_b) = noisy_line(|i, n| n - i);
    let center_index = values_a.len();
    values_a.append(&mut values_b);
    weights_a.append(&mut weights_b);
    let res = pava::Regression::new_radial(
        &values_a,
        &weights_a,
        center_index,
        pava::Ordering::Decreasing,
    );
    dump_svg("radial_decreasing", &values_a, &res.values);
}

#[test]
fn radial_increasing() {
    let (mut values_a, mut weights_a) = noisy_line(|i, n| n - i);
    let (mut values_b, mut weights_b) = noisy_line(|i, _| i);
    let center_index = values_a.len();
    values_a.append(&mut values_b);
    weights_a.append(&mut weights_b);
    let res = pava::Regression::new_radial(
        &values_a,
        &weights_a,
        center_index,
        pava::Ordering::Increasing,
    );
    dump_svg("radial_increasing", &values_a, &res.values);
}
