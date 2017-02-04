use liquidfun::box2d::common::math::Vec2;
use sdl2::rect::{Rect,Point};
use std::num;
// Coordinate struct used to convert between u32 pixels and box2d f32 meters. 
// One meter == 10px
// box2d (0.0,0.0) -> 
#[derive(Debug)]
pub struct Coords<T> {
  pub x: T,
  pub y: T
}
impl Coords<f32> {
  pub fn new(x: f32, y: f32) -> Coords<f32> {
    Coords { x: x, y: y }
  }
  pub fn from_vec(v: &Vec2) -> Coords<f32> {
    Coords {
      x: v.x,
      y: v.y
    }
  }
  pub fn convertu(&self) -> Coords<u32> {
    let x = self.x*10.0;
    let y = self.y*10.0;
    Coords {
      x: x as u32,
      y: y as u32
    }
  }
  pub fn converti(&self) -> Coords<i32> {
    let x = self.x*10.0;
    let y = self.y*10.0;
    Coords {
      x: x as i32,
      y: y as i32
    }
  }
  pub fn get_sdl_point(&self, offset: &Coords<f32>) -> Point {
    let self_pixels = self.converti();
    let offset_pixels = offset.converti();
    //println!("self {:?} pixels {:?}",self, self_pixels);
    //println!("offset {:?} pixels {:?}",offset, offset_pixels);
    let x = offset_pixels.x + self_pixels.x;
    let y = offset_pixels.y + self_pixels.y;
    Point::new(x, y)
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

pub trait ConvertToRect {
  fn to_rect(&self) -> Rect;
}
impl ConvertToRect for Vec<Coords<f32>> {
  fn to_rect(&self) -> Rect {

    Rect::from((50,50,10,10))
  }
}