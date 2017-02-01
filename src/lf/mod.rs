use liquidfun::box2d::collision::shapes::polygon_shape::*;
use liquidfun::box2d::common::math::*;
use liquidfun::box2d::dynamics::body::*;
use liquidfun::box2d::dynamics::fixture::*;
use liquidfun::box2d::dynamics::world::*;

mod coords;
use coords::Coords;

pub struct BoxDef {
    pos: (f32, f32),
    size: f32,
    density: f32, 
    friction: f32,
    restitution: f32
}
impl Default for BoxDef {
    fn default() -> BoxDef {
        BoxDef {
            pos: (0.0,0.0), size: 10.0, density: 1.0, friction: 0.3, restitution: 1.0
        }
    }
}
pub struct LFWorld {
    world: World
}
// 10px for 1 meter
impl LFWorld {
    pub fn new(&resolution: &(u16,u16) ) -> LFWorld {
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
        ground_box.set_as_box(resolution.0/2, 0.5);
        // Add the ground fixture to the ground body.
        ground_body.create_fixture_from_shape(&ground_box, 0.0);

        LFWorld {
            world: world
        }
    }
    pub fn createBox(&mut self, bdef: &BoxDef) -> Body {
        // Define the dynamic body. We set its position and call the body factory.
        let mut body_def = BodyDef::default();
        body_def.body_type = BodyType::DynamicBody;
        body_def.position.set(bdef.pos.0, bdef.pos.1);
        let body = self.world.create_body(&body_def);
        // Define another box shape for our dynamic body.
        let mut dynamic_box = PolygonShape::new();
        dynamic_box.set_as_box(bdef.size, bdef.size);
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
    pub fn test(&mut self) {

	// Prepare for simulation. Typically we use a time step of 1/60 of a
	// second (60Hz) and 10 iterations. This provides a high quality simulation
	// in most game scenarios.
	let time_step = 1.0 / 60.0;
	let velocity_iterations = 6;
	let position_iterations = 2;
    let body = self.createBox(&BoxDef::default());
	// This is our little game loop.
	for _ in 0..60 {

		// Instruct the world to perform a single step of simulation.
		// It is generally best to keep the time step and iterations fixed.
		self.world.step(time_step, velocity_iterations, position_iterations);

		// Now print the position and angle of the body.
        let position = Coords::new(body.get_position());
        
		let angle = body.get_angle();

        println!("{:?} angle: {:?}", position.convert(), angle);
	}
    }        
}

