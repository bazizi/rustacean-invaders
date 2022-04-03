use crate::vec2::Vec2;

pub struct SpaceShip
{
    x: f64,
    velocity: f64
}

impl SpaceShip {
    pub fn new() -> Self
    {
        Self{
            x : 0.,
            velocity: 5.
        }
    }

    pub fn update_position(&mut self, direction : Vec2)
    {
        self.x = self.x + self.velocity * direction.x;
    }

    pub fn get_x(&self) -> f64
    {
        self.x
    }
}