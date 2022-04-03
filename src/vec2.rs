    #[derive(std::marker::Copy)]
    pub struct Vec2
    {
        pub x: f64
    }

    impl Vec2
    {
        pub fn new() -> Self
        {
            Self{
                x : 0.
            }
        }        
    }

    impl Clone for Vec2
    {
        fn clone(&self) -> Self
        {
            Self{x: self.x}
        }
    }