extern crate sdl2;
extern crate liquidfun;

mod wrsdl;
mod lf;

use wrsdl::WrSdl;
use sdl2::pixels::Color;

fn main() {
    let resolution = (640u16,480u16);
    let mut ctx = WrSdl::new(); 
    let mut lfw = lf::LFWorld::new(&resolution);
    lfw.test();

    ctx.events.pump();
    loop {
        ctx.events.pump();
        if ctx.events.quit | ctx.events.key_escape {
            break;
        }
        ctx.clear(Color::RGBA(0,0,0,50));
        ctx.draw_box(Color::RGB(200,200,50),false);
        ctx.renderer.present();
    }
}
