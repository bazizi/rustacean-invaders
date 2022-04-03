#[derive(std::marker::Copy)]
pub struct Vec2
{
    pub x: f64,
    pub y: f64
}

impl Vec2
{
    pub fn new() -> Self
    {
        Self{
            x : 0.,
            y: 0.
        }
    }        
    
    pub fn magnitude(&self) -> f64
    {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn clamp(&mut self, top_left_bound: Vec2, bottom_right_bound: Vec2)
    {
        if self.x > bottom_right_bound.x
        {
            self.x = bottom_right_bound.x;
        }
        else if self.x < top_left_bound.x
        {
            self.x = top_left_bound.x;
        }

        if self.y > bottom_right_bound.y
        {
            self.y = bottom_right_bound.y;
        }
        else if self.y < top_left_bound.y
        {
            self.y = top_left_bound.y;
        }
    }
}

impl Clone for Vec2
{
    fn clone(&self) -> Self
    {
        Self{x: self.x, y: self.y}
    }
}

impl std::ops::Div<f64> for Vec2
{
    type Output = Vec2;

    fn div(self, rhs: f64) -> Self::Output
    {
        Vec2{
            x: self.x / rhs,
            y: self.y / rhs
        }
    }
}

impl std::ops::AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2)
    {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::SubAssign<Vec2> for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2)
    {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output
    {
        Vec2{
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl std::fmt::Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "({}, {})", &self.x, self.y)
    }
}