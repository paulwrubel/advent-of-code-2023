use core::fmt;

use nalgebra::{Matrix3, Vector3};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Point2D {
    x: f64,
    y: f64,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl From<(f64, f64)> for Point2D {
    fn from((x, y): (f64, f64)) -> Self {
        Self { x, y }
    }
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QuadraticEquation {
    a: f64,
    b: f64,
    c: f64,
}

impl QuadraticEquation {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Self { a, b, c }
    }

    pub fn from_points(points: &[Point2D; 3]) -> Result<Self, String> {
        let Point2D { x: x1, y: y1 } = points[0];
        let Point2D { x: x2, y: y2 } = points[1];
        let Point2D { x: x3, y: y3 } = points[2];

        let equation_matrix = // this comment breaks the line to make it readable
        Matrix3::new(x1 * x1, x1, 1.0, x2 * x2, x2, 1.0, x3 * x3, x3, 1.0);

        let y_vector = Vector3::new(y1, y2, y3);

        if let Some(coefficients) = equation_matrix.try_inverse().map(|inv| inv * y_vector) {
            Ok(Self {
                a: coefficients[0],
                b: coefficients[1],
                c: coefficients[2],
            })
        } else {
            Err(format!(
                "Points {:?} cannot form a quadratic equation",
                points
            ))
        }
    }

    pub fn solve_for_y(&self, x: f64) -> f64 {
        self.a * (x * x) + self.b * x + self.c
    }
}

impl fmt::Display for QuadraticEquation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "y = {}x^2 + {}x + {}", self.a, self.b, self.c)
    }
}
