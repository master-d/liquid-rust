use liquidfun::box2d::common::math::Vec2;
use sdl2::rect::Rect;

// Coordinate struct used to convert between u32 pixels and box2d f32 meters. 
// One meter == 10px
// box2d (0.0,0.0) -> 
#[derive(Debug)]
pub struct Coords<T> {
  x: T,
  y: T
}
impl Coords<f32> {
  pub fn new(v: &Vec2) -> Coords<f32> {
    Coords {
      x: v.x,
      y: v.y
    }
  }
  pub fn convert(&self) -> Coords<u32> {
    let x = self.x*10.0;
    let y = self.y*10.0;
    Coords {
      x: x as u32,
      y: y as u32
    }
  }
}
impl Coords<u32> {
  pub fn convert(&self) -> Coords<f32> {
    let x = self.x/10;
    let y = self.y/10;
    Coords {
      x: x as f32,
      y: y as f32
    }
  }
}

trait ConvertToRect {
  fn to_rect(&self) -> Rect;
}
impl ConvertToRect for Vec<Coords<f32>> {
  fn to_rect(&self) -> Rect {
    Rect::from((50,50,10,10));
  }
}