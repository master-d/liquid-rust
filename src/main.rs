extern crate sdl2;
extern crate liquidfun;

mod wrsdl;
mod lf;
mod coords;

use liquidfun::box2d::dynamics::body::Body;

use wrsdl::WrSdl;
use coords::Coords;
use sdl2::pixels::Color;
use lf::BoxDef;
use std::{thread, time};

fn main() {
    
    let resolution = (800u32,600u32);
    let mut ctx = WrSdl::new(resolution); 
    let mut lfw = lf::LFWorld::new(&resolution);
    let mut bvec: Vec<BoxDef> = Vec::new();

    // Typically we use a time step of 1/60 of a second (60Hz) and 10 iterations.
    let time_step = 1.0 / 60.0;
    let velocity_iterations = 6;
    let position_iterations = 2;
    let slp_millis = time::Duration::from_millis((time_step*1000.0) as u64);

    for x in 0u32..9u32 {
        let y = x +50;
        let mut bdef = BoxDef { pos: (y as f32,50.0), ..Default::default() };
        let body = lfw.create_box(&bdef);
        bdef.set_body(body);
        bvec.push(bdef);
    }

    loop {
        ctx.events.pump();
        if ctx.events.quit | ctx.events.key_escape {
            break;
        }
        // clear the screen
        ctx.renderer.set_draw_color(Color::RGBA(0,0,0,50));
        ctx.renderer.clear();
        // Instruct the world to perform a single step of simulation.
        // It is generally best to keep the time step and iterations fixed.
        lfw.world.step(time_step, velocity_iterations, position_iterations);
        thread::sleep(slp_millis);
        
        for bdef in &bvec {
            // Now print the position and angle of the body.
            lfw.draw_body(&bdef,&mut ctx);
        }
        ctx.renderer.present();
                
    }
}
