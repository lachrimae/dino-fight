use amethyst::core::math::Vector3;

// rectangles should have x1 < x2, y1 < y2.
pub struct Rectangle {
    pub x1: f32,
    pub x2: f32,
    pub y1: f32,
    pub y2: f32,
}

pub struct Vector {
    pub x: f32,
    pub y: f32,
}

pub fn rects_overlap(a: &Rectangle, b: &Rectangle) -> bool {
    let x_compatible = a.x1 < b.x2 && a.x2 > b.x1;
    let y_compatible = a.y1 < a.y2 && a.y2 > b.y1;
    x_compatible && y_compatible 
}

pub fn closest_point_on_rect(pos: &Vector3<f32>, rect: &Rectangle) -> Vector3<f32> {
    let x = if pos[0] < rect.x1 {
        rect.x1
    } else if pos[0] <= rect.x2 {
        pos[0]
    } else {
        rect.x2
    };

    let y = if pos[1] < rect.y1 {
        rect.y1
    } else if pos[1] <= rect.y2 {
        pos[1]
    } else {
        rect.y2
    };

    Vector3::new(x, y, 0.)
}

pub fn distance(pos: &Vector3<f32>, rect: &Rectangle) -> f32 {
    (pos - closest_point_on_rect(pos, rect)).norm()
}
