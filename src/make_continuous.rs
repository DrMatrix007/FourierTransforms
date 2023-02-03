use num::Complex;

use crate::function::Function;

pub struct Continuous {
    data: Vec<Complex<f64>>,
}
impl Continuous {
    pub fn new(data: Vec<Complex<f64>>) -> Self {
        Self {
            data
        }
    }
}
impl Function for Continuous {
    fn calculate(&self, x: f64) -> Complex<f64> {
        let pos = (x * self.data.len() as f64).floor() as usize;
        // print!("{} {}\r",pos,self.data.len());
        let i = self
            .data
            .get(pos)
            .expect("there should be values in Continuous::data");
        let left = x % (1.0 / self.data.len() as f64);
        match self.data.get(pos + 1) {
            Some(j) => i + left * self.data.len() as f64 * (j - i),
            None => *i,
        }
    }
}
