use colors_transform::{Color, Hsl};
use std::ops::{Add, Mul};

#[derive(Clone, Copy, Debug)]
pub struct Complex {
    real: f64,
    imaginary: f64,
}

impl Complex {
    pub fn from(real: f64, imaginary: f64) -> Self {
        Self { real, imaginary }
    }
    pub fn distance(&self, other: Complex) -> f64 {
        ((self.real - other.real).powi(2) + (self.imaginary - other.imaginary).powi(2)).sqrt()
    }
    pub fn norm(&self) -> f64 {
        (self.real.powi(2) + self.imaginary.powi(2)).sqrt()
    }
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real * rhs.real - self.imaginary * rhs.imaginary,
            imaginary: self.real * self.imaginary + rhs.real * rhs.imaginary,
        }
    }
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Self::Output {
        Self {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary,
        }
    }
}

const WIDTH: u32 = 32768;
const HEIGHT: u32 = 32768;

const X_MIN: f64 = -2.0;
const X_MAX: f64 = 1.0;
const Y_MIN: f64 = -1.5;
const Y_MAX: f64 = 1.5;

const MAX_ITERATION: u32 = 100;
const DISTANCE: f64 = 2.0;

const SCALE: f64 = 1.0;

fn main() {
    println!("Hello, world!");

    let mut image = image::ImageBuffer::new(WIDTH, HEIGHT);

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let x = x as f64;
        let y = y as f64;

        let mapped_x = X_MIN + (X_MAX - X_MIN) * x / WIDTH as f64;
        let mapped_y = Y_MIN + (Y_MAX - Y_MIN) * y / HEIGHT as f64;

        let mut z = Complex::from(0.0, 0.0);
        let c = Complex::from(SCALE * mapped_x, SCALE * mapped_y);
        let mut iteration = 0;
        while iteration < MAX_ITERATION && z.norm() <= DISTANCE {
            z = z * z + c;
            iteration += 1;
        }
        let rgb = Hsl::from(
            iteration as f32 / 2.0 + 245.0,
            100.0,
            if iteration < MAX_ITERATION {
                iteration as f32 / MAX_ITERATION as f32 * 50.0
            } else {
                0.0
            },
        )
        .to_rgb();
        let r = rgb.get_red() as u8;
        let g = rgb.get_green() as u8;
        let b = rgb.get_blue() as u8;
        *pixel = image::Rgb([r, g, b]);
    }

    image.save("fractal.png").unwrap();
}