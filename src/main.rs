use std::{f64::consts::PI, time::Instant};

use function::IntegrateArgs;
use num::{
    complex::{Complex64, ComplexFloat},
    Complex,
};
use renderer::FourierRenderer;
use sfml::{
    graphics::{Color, PrimitiveType, RenderStates, RenderTarget, Vertex, View},
    system::{Vector2, Vector2f},
    window::{Event, Style},
};

use crate::fourier_transform::FourierTransform;

pub mod fourier_transform;
pub mod function;
pub mod renderer;

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
    let size = Vector2 { x: 1.0, y: 1.0 };
    // println!("{}",(|x:f64|Complex::new(0.0,x*-2.0*PI).exp()).integrate(StepRange::new(0.0,1.0,0.00001)));
    let f = FourierTransform::new(-1000..1001,target, &args);

    let renderer = FourierRenderer::new(f);

    renderer.run();


    
}
