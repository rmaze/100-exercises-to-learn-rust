enum Shape {
    Circle { radius: f64 },
    Square { border: f64 },
    Rectangle { width: f64, height: f64 },
}

impl Shape {
    // Implement the `radius` method using `if let` or `let else`.
    pub fn radius(&self) -> f64 {
        if let Shape::Circle { radius } = self {
            return *radius;
        } else if let Shape::Square { .. } = self {
            panic!("Cannot get radius of a Square");
        } else if let Shape::Rectangle { .. } = self {
            panic!("Cannot get radius of a Rectangle");
        }
        panic!("Unknown shape");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle() {
        let _ = Shape::Circle { radius: 1.0 }.radius();
    }

    #[test]
    #[should_panic]
    fn test_square() {
        let _ = Shape::Square { border: 1.0 }.radius();
    }

    #[test]
    #[should_panic]
    fn test_rectangle() {
        let _ = Shape::Rectangle {
            width: 1.0,
            height: 2.0,
        }
        .radius();
    }
}
