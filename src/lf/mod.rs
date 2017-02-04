use liquidfun::box2d::collision::shapes::polygon_shape::*;
use liquidfun::box2d::common::math::*;
use liquidfun::box2d::dynamics::body::*;
use liquidfun::box2d::dynamics::fixture::*;
use liquidfun::box2d::dynamics::world::*;
use liquidfun::box2d::collision::shapes;

use sdl2::rect::{Rect,Point};
use sdl2::pixels::Color;

use coords::Coords;
use coords::ConvertToRect;
use wrsdl::WrSdl;

pub struct BoxDef {
    pub pos: (f32, f32),
    pub w: f32,
    pub h: f32,
    pub density: f32, 
    pub friction: f32,
    pub restitution: f32,
    pub body: Option<Body>
}
impl Default for BoxDef {
    fn default() -> BoxDef {
        BoxDef {
            pos: (0.0,0.0), w: 2.0, h: 1.0, density: 1.0, friction: 0.3, restitution: 0.1, body: None
        }
    }
}
impl BoxDef {
   pub fn set_body(&mut self, body: Body) {
        self.body = Some(body);
    }
}

pub struct LFWorld {
    pub world: World
}

impl LFWorld {
    pub fn new(&resolution: &(u32,u32) ) -> LFWorld {
        let (w,h) = resolution;
        let gravity = Vec2::new(0.0,-10.0);
        let mut world = World::new(&gravity);
        // Define the ground body.
        let mut ground_body_def = BodyDef::default();
        ground_body_def.position.set(0.0, 0.0);
        let ground_body = world.create_body(&ground_body_def);
        // Define the ground box shape.
        let mut ground_box = PolygonShape::new();
        // The extents are the half-widths of the box.
        ground_box.set_as_box(w as f32/2.0, 0.1);
        // Add the ground fixture to the ground body.
        ground_body.create_fixture_from_shape(&ground_box, 0.0);

        LFWorld {
            world: world
        }
    }
    pub fn create_box(&mut self, bdef: &BoxDef) -> Body {
        // Define the dynamic body. We set its position and call the body factory.
        let mut body_def = BodyDef::default();
        body_def.body_type = BodyType::DynamicBody;
        body_def.position.set(bdef.pos.0, bdef.pos.1);
        let body = self.world.create_body(&body_def);
        // Define another box shape for our dynamic body.
        let mut dynamic_box = PolygonShape::new();
        dynamic_box.set_as_box(bdef.w/2.0, bdef.h/2.0);
        // Define the dynamic body fixture.
        let mut fixture_def = FixtureDef::new(&dynamic_box);
        // Set the box density to be non-zero, so it will be dynamic.
        fixture_def.density = bdef.density;
        // Override the default friction.
        fixture_def.friction = bdef.friction;
        // set the bounciness of the box
        fixture_def.restitution = bdef.restitution;
        // Add the shape to the body.
        body.create_fixture(&fixture_def);
        body
    }
    pub fn draw_body(&mut self, bdef: &BoxDef, ctx: &mut WrSdl) {
        ctx.renderer.set_draw_color(Color::RGB(255,0,0));
        match bdef.body {
            Some(ref body) => {
                let center = Coords::from_vec(body.get_position()).converti();
                let wh = Coords::new(bdef.w,bdef.h).convertu();
                let rect: Rect = Rect::new(center.x,800-center.y, wh.x, wh.y);
                ctx.renderer.draw_rect(rect);
            },
            _ =>  println!("No body found")
        }
    }
    pub fn test(&mut self) {

	// Prepare for simulation. Typically we use a time step of 1/60 of a
	// second (60Hz) and 10 iterations. This provides a high quality simulation
	// in most game scenarios.
	let time_step = 1.0 / 60.0;
	let velocity_iterations = 6;
	let position_iterations = 2;
    let body = self.create_box(&BoxDef::default());
	// This is our little game loop.
	for _ in 0..60 {

		// Instruct the world to perform a single step of simulation.
		// It is generally best to keep the time step and iterations fixed.
		self.world.step(time_step, velocity_iterations, position_iterations);

		// Now print the position and angle of the body.
        let position = Coords::from_vec(body.get_position());
        
		let angle = body.get_angle();

        println!("{:?} angle: {:?}", position.converti(), angle);
	}
    }        
}

