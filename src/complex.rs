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
