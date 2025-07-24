/*
    Simulation demo for validation problem.
    It shows how to lead with uncertainties of measurements in data
    and obtain real model error characteristics _without_ accumulated
    measurements errors.
*/

use interp::interp;
use itertools_num::linspace;
use rand_distr::{Distribution, Normal, Uniform};
use statrs::function::erf::erf;
use std::f64::consts::{PI, SQRT_2};
use statrs::statistics::Statistics;

// Simulate what we measure for test and what we model
// for automated quality control system
struct Measure;
impl Measure {
    pub fn new(size: usize, low: f64, high: f64) -> Vec<f64> {
        // Simulate measurements
        // Uniform distribution here is just for fun and
        // to show why you cannot integrate measured points
        let uniform = Uniform::new_inclusive(low, high);
        let mut rng = rand::thread_rng();
        let mut data: Vec<f64> = Vec::new();
        data.resize_with(size, || uniform.sample(&mut rng));
        data
    }
    pub fn simulate(measure: &Vec<f64>, std: f64) -> Vec<f64> {
        // Draw out simulated data for model output and measurements
        measure
            .clone()
            .into_iter()
            .map(|val| {
                let normal = Normal::new(val, std).unwrap();
                normal.sample(&mut rand::thread_rng())
            })
            .collect()
    }

    pub fn mean_difference(model: &Vec<f64>, test: &Vec<f64>) -> f64 {
        model
            .iter()
            .zip(test)
            .map(|(a, b)| (a - b).abs())
            .collect::<Vec<f64>>()
            .iter()
            .sum::<f64>()
            / model.len() as f64
    }
}

fn clean_mae_calc(phi: f64, sigma_x: f64, sigma_y: f64) -> f64 {
    let sigma = (sigma_x.powi(2) + sigma_y.powi(2)).sqrt();
    //println!("{}", 2f64*sigma);
    let h_range: Vec<f64> = linspace::<f64>(0f64, 5f64 * sigma, 10_000)
        .into_iter()
        .collect();
    let phi_range: Vec<f64> = h_range
        .iter()
        .map(|&m| {
            // Closure containing main formula
            m * erf(m/ SQRT_2/ sigma) + SQRT_2 * sigma * (-0.5 * (m / sigma).powi(2)).exp() / PI.sqrt()
        })
        .collect::<Vec<f64>>();
    interp(&phi_range, &h_range, phi)
}



fn main() {
    let sigma: f64 = 0.07; // model own error std: what we want to know for validation
    let sigma_x: f64 = 0.18; // error std propagated from inputs (measurements)
    let sigma_y: f64 = 0.20; // error std for test data: each point has this uncertainty
    let sampling_size: usize = 1_000; // how many points we have
    {
        // Simulate data
        let b = Measure::new(sampling_size, 8.0, 12.0);

        // Make test data
        let y = Measure::simulate(&b, sigma_y);

        // Make model output without uncertainty at input:
        // it should contain only model's own errors
        let a = Measure::simulate(&b, sigma);

        // Make model output with uncertainty at input
        let x = Measure::simulate(&a, sigma_x);

        // Real MAE
        let mae = Measure::mean_difference(&a, &b);
        println!("MAE (real): {}", mae);

        // Usual way to calculate MAE
        let wrong_mae = Measure::mean_difference(&x, &y);
        println!("WrongMAE (usual way): {}", wrong_mae);
        /*
        println!(
            "Relative difference |WrongMAE - MAE| / MAE =  {} %",
            (wrong_mae - mae).abs() / mae * 100.0
        );
        */

        // The new method
        let clean_mae = clean_mae_calc(wrong_mae, sigma_x, sigma_y);
        println!("CleanMAE (our method) = {}", clean_mae);
        /*
        println!(
            "Relative difference |CleanMAE - MAE| / MAE =  {} %",
            (clean_mae - mae).abs() / mae * 100.0
        );
        */
        // Comparison
        println!(
            "|WrongMAE - MAE| / |CleanMAE - MAE| = {}",
            (wrong_mae - mae).abs() / (clean_mae - mae).abs()
        );

        // News: MAE -> STD of model own error distribution
        let model_std = 1.2535 * clean_mae;
        println!("Model STD: {}", model_std);
    }

    println!("\nDelta Sigma (obs) VS. Delta Sigma (real)");


    let a0 = Measure::new(sampling_size, 8.0, 12.0);
    let a1 = Measure::new(sampling_size, 9.0, 11.0);

    let a0_std = &a0.clone().population_std_dev();
    let a1_std = &a1.clone().population_std_dev();
    
    println!("STD (real): {:?} -> {:?}, diff: {}", a0_std, a1_std, a0_std - a1_std);

    let x0 = Measure::simulate(&a0, 0.2f64);
    let x1 = Measure::simulate(&a1, 0.2f64);

    let x0_std = &x0.clone().population_std_dev();
    let x1_std = &x1.clone().population_std_dev();

    println!("STD (obs): {:?} -> {:?}, diff: {}", x0_std, x1_std, x0_std - x1_std);


}
