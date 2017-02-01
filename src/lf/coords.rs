use liquidfun::box2d::common::math::Vec2;

// Coordinate struct used to convert between u32 pixels and box2d f32 meters. 
// One meter == 10px
// box2d (0.0,0.0) -> 
pub struct Coords<T> {
  x: T,
  y: T
}
impl <T: f32> Coords<T> {
  pub fn new(v: &Vec2) -> Coords<f32> {
    Coords {
      x: v.x,
      y: v.y
    }
  }
  pub fn convert(&self) -> Coords<u32> {
    Coords {
      x: self.x*10 as u32,
      y: self.y*10 as u32
    }
  }
}
impl <T: u32> Coords<T> {
  pub fn convert(&self) -> Coords<f32> {
    Coords {
      x: self.x/10 as f32,
      y: self.y/10 as f32
    }
  }
}