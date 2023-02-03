use std::{f64::consts::PI, time::Instant};

use function::IntegrateArgs;
use num::{
    complex::{Complex64, ComplexFloat},
    traits::Pow,
    Complex,
};
use sfml::{
    graphics::{Color, PrimitiveType, RenderStates, RenderTarget, Vertex, View},
    system::{Vector2, Vector2f},
    window::{Event, Style},
};

use crate::fourier_transform::FourierTransform;

pub mod fourier_transform;
pub mod function;

fn target(x: f64) -> Complex<f64> {
    // Complex::new(
    //     if x > 2.0/3.0 { 1.0 } else { -1.0 },
    //     if x > 1.0/3.0 { 1.0 } else { -1.0 },
    // )
    let x = PI*x*2.0;

    Complex::new(
        if x.cos() < 0.0{ -1.0}else {1.0},
        if x.sin() < 0.0{ -1.0}else {1.0},
    )
}

fn main() {
    let mut window =
        sfml::graphics::RenderWindow::new((800, 800), "nice", Style::CLOSE, &Default::default());
    window.set_framerate_limit(140);
    window.set_view(&sfml::graphics::View::new(
        Vector2f::new(0.0, 0.0),
        Vector2f::new(3.0, 3.0),
    ));

    let is_on_camera = false;
    let size = Vector2 { x: 0.005, y: 0.005 };
    // println!("{}",(|x:f64|Complex::new(0.0,x*-2.0*PI).exp()).integrate(StepRange::new(0.0,1.0,0.00001)));
    let range = -500..501;
    let mut f = FourierTransform::new(range);
    let args = IntegrateArgs {
        start: 0.0,
        end: 1.0,
        dx: 0.0001,
    };
    let speed = 0.05;


    f.learn(target, &args);

    let mut array = Vec::new();
    let zero = Vector2f::new(0.0, 0.0);
    let background = Color::rgb(69, 69, 69);
    let mut start = Instant::now();
    let mut x = 0.0;
    while window.is_open() {
        while let Some(e) = window.poll_event() {
            if let Event::Closed = e {
                window.close()
            }
        }
        let mut vec = f.get_data().clone().into_iter().collect::<Vec<_>>();
        vec.sort_by(|x, y| x.1.abs().total_cmp(&y.1.abs()).reverse());
        x += (Instant::now() - start).as_secs_f64() * speed;
        start = Instant::now();
        let mut last = Complex64::default();
        array.push(Vertex::new(
            Vector2::new(last.re as f32, last.im as f32),
            Color::WHITE,
            zero,
        ));
        for (i, (pow, c)) in vec.iter().enumerate() {
            last += c * Complex::new(0.0, 2.0 * PI * x * *pow as f64).exp();
            array.push(Vertex::new(
                Vector2::new(last.re as f32, last.im as f32),
                Color::rgba(255, 255, 255, 255),
                zero,
            ));
        }
        drop(vec);
        let a = array.last().unwrap();
        print!("x:{x}\r");
        if is_on_camera{

            window.set_view(&View::new(a.position, size));
        }
        window.draw_primitives(
            array.as_slice(),
            PrimitiveType::LINE_STRIP,
            &RenderStates::default(),
        );
        window.display();
        window.clear(background);
        array.clear();
    }
}
