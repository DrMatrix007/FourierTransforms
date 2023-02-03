use std::{
    collections::{HashMap, LinkedList},
    iter::Map, time::Instant, f64::consts::PI,
};

use num::{complex::Complex64, Complex};
use sfml::{graphics::{RenderWindow, RenderTarget, Color, PrimitiveType, RenderStates, View, Vertex}, window::{Style, Event}, system::{Vector2f, Vector2}};

use crate::fourier_transform::FourierTransform;

struct TransformRenderState {
    pub last: LinkedList<Complex64>,
}

struct Renderer {
    window: RenderWindow,
    transforms: Vec<(FourierTransform, TransformRenderState)>,
}


fn create_window() -> RenderWindow {
    let mut window = sfml::graphics::RenderWindow::new(
        (800, 800),
        "nice",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_framerate_limit(144);
    window.set_view(&sfml::graphics::View::new(
        Vector2f::new(0.0, 0.0),
        Vector2f::new(3.0, 3.0),
    ));
    window
}

impl Renderer {
    fn new() -> Self {
        Self {
            window: create_window(),
            transforms: Default::default()
        }
    
    }


    fn run(&mut self)  {
        let mut array = Vec::new();
    let zero = Vector2f::new(0.0, 0.0);
    let background = Color::rgb(69, 69, 69);
    let mut start = Instant::now();
    let mut x = 0.0;
    let speed = 0.1;

    while self.window.is_open() {
        while let Some(e) = self.window.poll_event() {
            if let Event::Closed = e {
                self.window.close()
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
        for (pow, c) in vec.iter() {
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

        self.window.draw_primitives(
            array.as_slice(),
            PrimitiveType::LINE_STRIP,
            &RenderStates::default(),
        );
        self.window.display();
        self.window.clear(background);
        array.clear();
    }
    }
}
