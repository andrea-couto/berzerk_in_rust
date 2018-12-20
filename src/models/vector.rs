/// Vector is used to hold x and y position for game objects
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Self {
        Vector { x: x, y: y }
    }

}
