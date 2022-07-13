pub trait Area {
    fn area(&self) -> f32;
}

pub struct Circle {
    pub radius:f32
}

impl Area for Circle {
    fn area(&self) -> f32 {
        std::f32::consts::PI * self.radius * self.radius
    }
}

pub struct Square {
    pub side:f32
}

impl Area for Square {
    fn area(&self) -> f32 {
        self.side * self.side
    }
}

pub struct Triangle {
    pub side:f32
}

impl Area for Triangle {
    fn area(&self) -> f32 {
        1.732/4.0 * self.side * self.side
    }
}

pub fn get_area<T: Area> (shape: &T) -> f32 {
    shape.area()
}