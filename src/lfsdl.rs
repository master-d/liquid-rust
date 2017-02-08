use liquidfun::box2d::common::math::Vec2;
use sdl2::rect::{Rect,Point};
use sdl2::pixels::Color;
use std::num;

use lf::LFWorld;
use wrsdl::WrSdl;
use lf::BoxDef;

pub struct LfSdl<'window> {
  pub lf: LFWorld,
  pub sdl: WrSdl<'window>
}
impl <'window> LfSdl<'window> {
      pub fn draw_body(&mut self, bdef: &BoxDef) {
        let (r,g,b) = bdef.color;
        self.sdl.renderer.set_draw_color(Color::RGB(r,g,b));
        match bdef.body {
            Some(ref body) => {
                let center = Coords::from_vec(body.get_position()).converti();
                let center_y = self.sdl.resolution.1 as i32 - center.y as i32;
                let wh = Coords::new(bdef.w,bdef.h).convertu();
                let rect: Rect = Rect::new(center.x,center_y, wh.x, wh.y);
                self.sdl.renderer.draw_rect(rect);
            },
            _ =>  println!("No body found")
        }
    }
}

// Coordinate struct used to convert between u32 pixels and box2d f32 meters. 
// One meter == 10px
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
    let x = self.x*5.0;
    let y = self.y*5.0;
    Coords {
      x: x as u32,
      y: y as u32
    }
  }
  pub fn converti(&self) -> Coords<i32> {
    let x = self.x*5.0;
    let y = self.y*5.0;
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
    let x = self.x/5;
    let y = self.y/5;
    Coords {
      x: x as f32,
      y: y as f32
    }
  }
}

