// rectangles should have x1 < x2, y1 < y2.
pub struct Rectangle {
    pub x1: f32,
    pub x2: f32,
    pub y1: f32,
    pub y2: f32,
}

pub fn rects_overlap(a: &Rectangle, b: &Rectangle) -> bool {
    let x_compatible = a.x1 < b.x2 && a.x2 > b.x1;
    let y_compatible = a.y1 < a.y2 && a.y2 > b.y1;
    x_compatible && y_compatible
}
