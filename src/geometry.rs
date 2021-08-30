use std::f32::NAN;

// rectangles should have x1 < x2, y1 < y2.
pub struct Rectangle {
    pub x1: f32,
    pub x2: f32,
    pub y1: f32,
    pub y2: f32,
}

pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub struct DVector {
    pub magnitude: f32,
    pub direction: Point,
}

pub fn rects_overlap(a: &Rectangle, b: &Rectangle) -> bool {
    let x_compatible = a.x1 < b.x2 && a.x2 > b.x1;
    let y_compatible = a.y1 < a.y2 && a.y2 > b.y1;
    x_compatible && y_compatible
}

pub fn distance(p: &[f32], r: &Rectangle) -> DVector {
    let dx = if p[0] < r.x1 {
        r.x1 - p[0]
    } else if p[0] <= r.x2 {
        0.
    } else {
        r.x2 - p[0]
    };
    let dy = if p[1] < r.y1 {
        r.y1 - p[1]
    } else if p[1] <= r.y2 {
        0.
    } else {
        r.y2 - p[1]
    };

    let magnitude = (dx.powi(2) + dy.powi(2)).sqrt();
    let direction = if magnitude == 0. || magnitude == NAN {
        Point { x: 1., y: 0. }
    } else {
        Point { x: dx / magnitude, y: dy / magnitude }
    };

    DVector {
        magnitude,
        direction
    }
}
