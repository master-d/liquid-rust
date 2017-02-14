use liquidfun::box2d::common::math::Vec2;
use sdl2::rect::{Rect,Point};
use sdl2::pixels::Color;
use sdl2::gfx::primitives::DrawRenderer;
use std::num;

use lf::LFWorld;
use wrsdl::WrSdl;
use lf::BoxDef;

static PPM: u32 = 10;

pub struct LfSdl<'window> {
  pub lf: LFWorld,
  pub sdl: WrSdl<'window>
}
impl <'window> LfSdl<'window> {
      pub fn draw_body(&mut self, bdef: &BoxDef) {
        //self.sdl.renderer.set_draw_color(Color::RGB(r,g,b));
        match bdef.body {
            Some(ref body) => {
                //let (r,g,b) = bdef.color;
                //self.sdl.texture.set_color_mod(r,g,b);

                let center = Coords::from_vec(body.get_position()).converti();
                let wh = Coords::new(bdef.w,bdef.h).convertu();
                let center_y = self.sdl.resolution.1 as i32 - center.y as i32 - wh.y as i32/2;
                let rect: Rect = Rect::new(center.x-wh.x as i32/2,center_y, wh.x, wh.y);
                let clip: Rect = Rect::new(0,0, wh.x, wh.y);
                //let center_pt = Point::new(wh.x as i32, wh.y as i32);
                let angle = 360f64-(body.get_angle()as f64*180f64)/::std::f64::consts::PI;
                self.sdl.renderer.copy_ex(&self.sdl.texture, Some(clip), Some(rect), angle as f64,None,false,false);
            },
            _ =>  println!("No body found")
        }
    }
    pub fn draw_particle_as_box(&mut self, pos: &Vec2) {
      self.sdl.renderer.set_draw_color(Color::RGBA(255,255,255,255));
      let center = Coords::from_vec(pos).converti();
      let center_y = self.sdl.resolution.1 as i32 - center.y as i32;
      let wh = Coords{ x: 3, y: 3 };
      let clip: Rect = Rect::new(20,0, wh.x, wh.y);
      let rect: Rect = Rect::new(center.x,center_y, wh.x, wh.y);
      self.sdl.renderer.copy(&self.sdl.texture, Some(clip), Some(rect));
    }
    pub fn draw_particle(&mut self, pos: &Vec2) {
      let center = Coords::from_vec(pos).converti();
      let center_y = self.sdl.resolution.1 as i32 - center.y as i32;
      self.sdl.renderer.set_draw_color(Color::RGBA(0,128,255,50));
      self.sdl.renderer.draw_point(Point::new(center.x,center_y));
      //self.sdl.renderer.fill_rect(Rect::new(center.x,center_y,1,1));
      /*
      self.sdl.renderer.circle(
        center.x as i16,
        center_y as i16,
        1,
        Color::RGBA(0,128,255,200));
      */

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
    let x = self.x*PPM as f32;
    let y = self.y*PPM as f32;
    Coords {
      x: x as u32,
      y: y as u32
    }
  }
  pub fn converti(&self) -> Coords<i32> {
    let x = self.x*PPM as f32;
    let y = self.y*PPM as f32;
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
    let x = self.x/PPM;
    let y = self.y/PPM;
    Coords {
      x: x as f32,
      y: y as f32
    }
  }
}

