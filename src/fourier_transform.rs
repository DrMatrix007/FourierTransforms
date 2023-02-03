use std::{collections::HashMap, f64::consts::PI, ops::Range};

use num::{complex::Complex64, Complex};

use crate::{function::{Function, IntegrateArgs}};
#[derive(Debug)]
pub struct FourierTransform {
    data: HashMap<i128, Complex<f64>>,
}

impl FourierTransform {
    pub fn new(range: Range<i128>) -> Self {
        Self {
            data: range
                .into_iter()
                .map(|x| (x, Complex64::default()))
                .collect(),
        }
    }
    pub fn learn(&mut self, f: impl Function, args: &IntegrateArgs) {
        for (pow, c) in self.data.iter_mut() {
            *c = (|x: f64| {
                 f.calculate(x) * (Complex::new(0.0, -2.0 * PI * (*pow as f64) * x).exp())
            })
            .integrate(args) ;
            

        }
    }
    pub fn get_data(&self) -> &HashMap<i128,Complex64> {
        &self.data
    }
}

impl Function for FourierTransform {
    fn calculate(&self, x: f64) -> num::Complex<f64> {
        self.data
            .iter()
            .map(|(pow, c)| c * (Complex::new(0.0, *pow as f64 * x * 2.0 * PI).exp()))
            .sum()
    }
}
