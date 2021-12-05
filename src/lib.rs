use std::error::Error;
use std::io;
//use std::env;
use serde::Deserialize;
use num::pow;

#[derive(Debug, Deserialize)]
pub struct Record {
    open: f64,
    close: f64,
}

pub fn read_data() -> Result<[Vec<f64>; 2], Box<dyn Error>> {
    let mut open_data: Vec<f64> = Vec::new();
    let mut close_data: Vec<f64> = Vec::new();
    let mut rdr = csv::Reader::from_reader(io::stdin());

    for result in rdr.deserialize() {
        let record: Record = result?;
        open_data.push(record.open);
        close_data.push(record.close);
    }

    let data: [Vec<f64>; 2] = [open_data, close_data];

    Ok(data)
}

#[derive(Copy, Clone)]
pub struct LinearRegression{
    pub coefficient: f64,
    pub intercept: f64,
}

impl LinearRegression{

    pub fn fit(&mut self, x: &Vec<f64>, y: &Vec<f64>){
        self.coefficient = self.estimate_coefficient(x, y);
        self.intercept = self.compute_intercept(x, y);
    }

    pub fn predict(&self, x: &Vec<f64>) -> Vec<f64> {
        let x_times_coef = vec_multiply(x, self.coefficient);
        vec_add(&x_times_coef, self.intercept)
    }

    pub fn r2(&self, y_true: &Vec<f64>, y_pred: &Vec<f64>) -> f64{
        let y_avg = mean(y_true);
        let mut residual_number_of_squares: f64 = 0.0;
        let mut total_number_of_squares: f64 = 0.0;

        for (i, _j) in y_true.iter().enumerate() {
            residual_number_of_squares = residual_number_of_squares + pow(y_true[i] - y_pred[i], 2);
            total_number_of_squares = total_number_of_squares + pow(y_true[i] - y_avg, 2);
        }

        1.0 - (residual_number_of_squares / total_number_of_squares)
    }

    pub fn compute_intercept(&self, x: &Vec<f64>, y: &Vec<f64>) -> f64 {
        let x_avg = mean(x);
        let mul = self.coefficient * x_avg;

        mean(y) - mul
    }
    
    pub fn estimate_coefficient(&self, x: &Vec<f64>, y: &Vec<f64>) -> f64 {
        let mut numerator: f64 = 0.0;
        let mut denominator: f64 = 0.0;

        for (i, _j) in x.iter().enumerate(){
            let x_val = x[i];
            let y_val = y[i];
            let x_avg = mean(x);
            let y_avg = mean(y);

            numerator = numerator + ((x_val - x_avg) * (y_val - y_avg));
            denominator = denominator + pow(x_val - x_avg, 2);
        }

        numerator / denominator
    }
}

pub fn mean(list: &Vec<f64>) -> f64 {
    let sum: f64 = list.iter().sum();

    sum / (list.len() as f64)
}

pub fn vec_multiply(x: &Vec<f64>, y: f64) -> Vec<f64> {
    let mut output: Vec<f64> = Vec::new();

    for (i, _j) in x.iter().enumerate() {
        output.push(x[i] * y);
    }

    output
}

pub fn vec_add(x: &Vec<f64>, y: f64) -> Vec<f64>{
    let mut output: Vec<f64> = Vec::new();

    for (i, _j) in x.iter().enumerate() {
        output.push(x[i] + y);
    }

    output
}