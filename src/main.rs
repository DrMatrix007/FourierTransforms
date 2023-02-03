
use function::IntegrateArgs;
use num::{
    Complex,
};
use renderer::FourierRenderer;

use crate::fourier_transform::FourierTransform;

pub mod fourier_transform;
pub mod function;
pub mod renderer;
pub mod make_continuous;

fn target(x: f64) -> Complex<f64> {
    Complex::new(
        if x > 2.0/3.0 { 1.0 } else { -1.0 },
        if x > 1.0/3.0 { 1.0 } else { -1.0 },
    )
    // let x = PI*x*3.5;

    // Complex::new(
    //     if x.cos() < 0.0{ -1.0}else {1.0},
    //     if x.sin() < 0.0{ -1.0}else {1.0},
    // )

    // Complex64::new(
        // x.cos(),
        // x.sin()
    // )
}

fn main() {

    

    let args = IntegrateArgs {
        start: 0.0,
        end: 1.0,
        dx: 0.0001,
    };
    let f = FourierTransform::new(-1000..1001,target, args);

    let renderer = FourierRenderer::new(f);

    renderer.run();


    
}
