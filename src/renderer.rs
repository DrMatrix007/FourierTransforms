use std::{collections::VecDeque, f64::consts::PI, time::Instant};

use num::{
    complex::{Complex64, ComplexFloat},
    Complex,
};
use sfml::{
    graphics::{Color, PrimitiveType, RenderStates, RenderTarget, RenderWindow, Vertex, View},
    system::{Vector2, Vector2f},
    window::{Event, Style},
};

use crate::{fourier_transform::FourierTransform, function::Function};

#[derive(Default)]
struct TransformRenderState {
    pub last: Vec<Vertex>,
}

impl TransformRenderState {
    pub fn push(&mut self, c: Vertex) {
        self.last.push(c);
    }

    pub fn clear(&mut self) {
        self.last.clear()
    }

    pub(crate) fn as_slice(&self) -> &[Vertex] {
        self.last.as_slice()
    }
}

pub struct FourierRenderer {
    window: RenderWindow,
    transforms: FourierTransform,
}

fn create_window() -> RenderWindow {
    let mut window =
        sfml::graphics::RenderWindow::new((800, 800), "nice", Style::CLOSE, &Default::default());
    window.set_framerate_limit(144);
    window.set_view(&sfml::graphics::View::new(
        Vector2f::new(0.0, 0.0),
        Vector2f::new(3.0, 3.0),
    ));
    window
}

impl FourierRenderer {
    pub fn new(transforms: FourierTransform) -> Self {
        Self {
            window: create_window(),
            transforms,
        }
    }

    pub fn run(mut self) {
        let zero = Vector2f::new(0.0, 0.0);
        let background = Color::rgb(69, 69, 69);
        let mut start = Instant::now();
        let mut x = 0.0;
        let speed = 0.1;
        let mut state = TransformRenderState::default();

        let mut transforms = self
            .transforms
            .get_data()
            .clone()
            .into_iter()
            .collect::<Vec<_>>();
        transforms.sort_by(|x, y| x.1.abs().total_cmp(&y.1.abs()).reverse());

        let mut array = Vec::new();

        while self.window.is_open() {
            while let Some(e) = self.window.poll_event() {
                if let Event::Closed = e {
                    self.window.close()
                }
            }
            if x>=2.0 {
                x= 0.0;
                state.clear();
            }
            x += (Instant::now() - start).as_secs_f64() * speed;
            
            start = Instant::now();
            let mut last = Complex64::default();
            array.push(Vertex::new(
                Vector2::new(last.re as f32, last.im as f32),
                Color::WHITE,
                zero,
            ));
            for (pow, c) in transforms.iter() {
                last += c * Complex::new(0.0, 2.0 * PI * x * *pow as f64).exp();
                array.push(Vertex::new(
                    Vector2::new(last.re as f32, last.im as f32),
                    Color::rgba(255, 255, 255, 255),
                    zero,
                ));
            }

            let mut a = *array.last().unwrap();
            a.color = Color::RED;
            state.push(a);
            
            print!("x:{x}\r");

            self.window.draw_primitives(
                array.as_slice(),
                PrimitiveType::LINE_STRIP,
                &RenderStates::default(),
            );
            self.window.draw_primitives(
                state.as_slice(),
                PrimitiveType::LINE_STRIP,
                &RenderStates::default(),
            );
            
            self.window.display();
            self.window.clear(background);
            array.clear();
        }
    }
}
