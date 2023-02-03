use num::Complex;

#[derive(Debug)]
pub struct IntegrateArgs {
    pub start: f64,
    pub end: f64,
    pub dx: f64,
}

pub trait Function {
    fn calculate(&self, x: f64) -> Complex<f64>;
    fn integrate(&self, args: &IntegrateArgs) -> Complex<f64> {
        let mut sum = Complex::default();
        let mut x = args.start;

        while x <= args.end {
            sum += self.calculate(x);
            x += args.dx;
        }

        sum *args.dx
    }
}

impl<T: Fn(f64) -> Complex<f64>> Function for T {
    fn calculate(&self, x: f64) -> Complex<f64> {
        self(x)
    }
}
// impl Function for fn(f64)->Complex<f64> {
//     fn calculate(&self, x: f64) -> Complex<f64> {
//         self(x)
//     }
// }
