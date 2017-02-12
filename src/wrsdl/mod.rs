use std::path::Path;

use sdl2::pixels::{Color,PixelFormatEnum};
use sdl2::event::Event;
use sdl2::rect::{Rect,Point};
use sdl2::render::{Renderer,TextureAccess,Texture};
use sdl2::timer::Timer;
use sdl2::{ Sdl, EventPump, surface };
use sdl2::video::Window; 
use std::time::Duration;

pub struct WrSdl<'window> {
    ctx: Sdl,
	pub renderer: Renderer<'window>,
    pub events: Events,
    pub resolution: (u32, u32),
    pub texture: Texture
}
impl<'window> WrSdl<'window> {
    pub fn new(resolution: (u32,u32)) -> WrSdl<'window> {
        let ctx = ::sdl2::init().unwrap();
        let video_subsystem = ctx.video().unwrap();
        let window = video_subsystem.window("sdl2", resolution.0, resolution.1).position_centered().build().unwrap();
        let mut renderer = window.renderer().accelerated().build().unwrap();
        let events = Events::new(ctx.event_pump().unwrap());
        let surface = surface::Surface::load_bmp("/home/yi/git/liquid-rust/src/assets/bricks.bmp").unwrap();
        let mut texture = renderer.create_texture_from_surface(surface).unwrap(); //().unwrap();
        //let mut texture = renderer.create_texture(PixelFormatEnum::RGBA8888,TextureAccess::Static,10,10).unwrap();
        //texture.set_draw_color(Color::RGB(255,0,0));
        //texture.set_color_mod(255,0,0);
        WrSdl {
            ctx: ctx,
            renderer: renderer,
            events: events,
            resolution: resolution,
            texture: texture
        }
    }
    
}

pub struct Events {
    pump: EventPump,
    pub quit: bool,
    pub key_escape: bool,
    pub key_space: bool
}
impl Events {
    pub fn new(pump: EventPump) -> Events {
        Events {
            pump: pump,
            quit: false,
            key_escape: false,
            key_space: false
        }
    }
    pub fn pump(&mut self) {
        use sdl2::keyboard::Keycode::*;

        for event in self.pump.poll_iter() {
            match event {
                Event::Quit { .. } => self.quit = true,
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Escape) => self.key_escape = true,
                    Some(Space) => self.key_space = true,
                    _ => {}
                },
                Event::KeyUp { keycode, .. } => match keycode {
                    Some(Escape) => self.key_escape = false,
                    Some(Space) => self.key_space = false,
                    _ => {}
                },
                _ => {}
            }
        }
    }
}


