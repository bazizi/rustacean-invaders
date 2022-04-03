use crate::vec2::Vec2;
use crate::webutils::*;

pub struct SpaceShip
{
    position: Vec2,
    velocity: Vec2,
    mass: f64,
    acceleration: Vec2,
    max_velocity: f64,
}

impl SpaceShip {
    pub fn new(x: f64, y: f64) -> Self
    {
        Self{
            position : Vec2 { x, y },
            velocity: Vec2::new(),
            mass: 5.,
            acceleration: Vec2::new(),
            max_velocity: 20.,
            
        }
    }

    pub fn add_force(&mut self, force : Vec2)
    {
        self.acceleration = force / self.mass;
 
        self.position = self.position + self.velocity;
        if self.velocity.magnitude() < self.max_velocity
        {
            self.velocity += self.acceleration;      
        }

        self.velocity = self.velocity / 2.; // drag

        self.position.clamp(Vec2{x: 0., y: 0.}, Vec2{x: 750., y: 600.});

        log(&format!("spaceship: (position={}, velocity={})", self.position, self.velocity));
    }

    pub fn get_position(&self) -> Vec2
    {
        self.position
    }
}