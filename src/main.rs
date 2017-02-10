extern crate sdl2;
extern crate liquidfun;
extern crate rand;

mod wrsdl;
mod lf;
mod lfsdl;

use rand::Rng;
use liquidfun::box2d::dynamics::body::Body;
use sdl2::pixels::Color;

use wrsdl::WrSdl;
use lf::BoxDef;
use std::{thread, time};

fn main() {
    let mut rng = rand::thread_rng();
    let resolution = (800u32,600u32);
    let wrsdl = WrSdl::new(resolution); 
    let lfw = lf::LFWorld::new();
    
    let mut ctx = lfsdl::LfSdl {
        sdl: wrsdl,
        lf: lfw 
    };
    let mut bvec: Vec<BoxDef> = Vec::new();

    // Typically we use a time step of 1/60 of a second (60Hz) and 10 iterations.
    let time_step = 1.0 / 60.0;
    let velocity_iterations = 6;
    let position_iterations = 2;
    let slp_millis = time::Duration::from_millis((time_step*1000.0) as u64);

    // add ground body to bvec
    bvec.push(ctx.lf.create_ground(resolution.0));
    // draw dam
    bvec.append(&mut ctx.lf.create_dam(50.0));

    loop {
        ctx.sdl.events.pump();
        if ctx.sdl.events.quit | ctx.sdl.events.key_escape {
            break;
        } 
        else if ctx.sdl.events.key_space {
            
            let mut bdef = BoxDef { pos: (50.0,50.0), w: 2.0, h: 2.0,
                //color: (rng.gen::<u8>(),rng.gen::<u8>(),rng.gen::<u8>()),
                ..Default::default() 
            };
            let body = ctx.lf.create_body(&bdef);
            bdef.set_body(body);
            bvec.push(bdef);
        }
        // clear the screen
        ctx.sdl.renderer.set_draw_color(Color::RGB(0,0,0));
        //ctx.sdl.renderer.set_draw_color(Color::RGB(255,255,255));
        ctx.sdl.renderer.clear();
        // Instruct the world to perform a single step of simulation.
        // It is generally best to keep the time step and iterations fixed.
        ctx.lf.world.step(time_step, velocity_iterations, position_iterations);
        thread::sleep(slp_millis);
        
        for bdef in &bvec {
            // Now print the position and angle of the body.
            ctx.draw_body(&bdef);
        }
        ctx.sdl.renderer.present();
        
    }
}
