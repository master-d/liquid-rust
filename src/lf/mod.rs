use liquidfun::box2d::collision::shapes::polygon_shape::*;
use liquidfun::box2d::common::math::*;
use liquidfun::box2d::dynamics::body::*;
use liquidfun::box2d::dynamics::fixture::*;
use liquidfun::box2d::dynamics::world::*;
use liquidfun::box2d::collision::shapes;
use liquidfun::box2d::particle::particle_system::{ParticleSystemDef,ParticleSystem};
use liquidfun::box2d::particle::particle_color::ParticleColor;
use liquidfun::box2d::particle::particle_group::{ParticleGroup,B2ParticleGroup};
use liquidfun::box2d::particle::*;
use liquidfun::box2d::particle::{particle_system,particle_group,B2ParticleDef,ParticleDef};


use sdl2::rect::{Rect,Point};
use sdl2::pixels::Color;

use wrsdl::WrSdl;

pub struct BoxDef {
    pub pos: (f32, f32),
    pub w: f32,
    pub h: f32,
    pub density: f32, 
    pub friction: f32,
    pub restitution: f32,
    pub color: (u8, u8, u8),
    pub dynamic: bool,
    pub body: Option<Body>
}
impl Default for BoxDef {
    fn default() -> BoxDef {
        BoxDef {
            pos: (0.0,0.0), w: 2.0, h: 2.0, density: 0.1, 
            friction: 0.5, restitution: 0.1, dynamic: true,
            color: (225,225,225), body: None
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
    pub fn new() -> LFWorld {
        let gravity = Vec2::new(0.0,-10.0);
        let mut world = World::new(&gravity);
        LFWorld {
            world: world
        }
    }
    pub fn create_body(&mut self, bdef: &BoxDef) -> Body {
        // Define the dynamic body. We set its position and call the body factory.
        let mut body_def = BodyDef::default();
        body_def.body_type = if bdef.dynamic 
        { BodyType::DynamicBody } else { BodyType::StaticBody };
        body_def.position.set(bdef.pos.0, bdef.pos.1);
        let body = self.world.create_body(&body_def);
        // Define another box shape for our dynamic body.
        let mut shape = PolygonShape::new();
        shape.set_as_box(bdef.w/2.0, bdef.h/2.0);
        // Define the dynamic body fixture.
        let mut fixture_def = FixtureDef::new(&shape);
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
    pub fn create_ground(&mut self, width: u32) ->BoxDef {
        let mut gdef = BoxDef { 
            pos: (width as f32/20.0,1.0), 
            w: width as f32/10.0, 
            h: 1.0, 
            color: (0,255,0),
            dynamic: false, ..Default::default() 
        };
        let ground = self.create_body(&gdef);
        gdef.set_body(ground);
        gdef
    }
    pub fn create_back_wall(&mut self, width: u32) ->BoxDef {
        let mut bw = BoxDef { 
            pos: (width as f32/10.0,0.0), 
            w: 1.0, 
            h: 50.0, 
            color: (0,255,0),
            dynamic: false, ..Default::default() 
        };
        let wall = self.create_body(&bw);
        bw.set_body(wall);
        bw
    }
    pub fn create_dam(&mut self, xpos: f32) -> Vec<BoxDef> {
        let mut damn = Vec::new();
        for x in 0..11 {
            for y in x..11 {
                if (x != 10 || y !=10) {
                let mut bdef = BoxDef{ 
                    pos: (xpos+2.0*y as f32, 2.0*x as f32),
                    w: 2.0,
                    h: 2.0,
                    restitution: 0.0,
                    density: 5.0,
                    friction: 0.9,
                    ..Default::default()
                };
                let body = self.create_body(&bdef);
                bdef.set_body(body);
                damn.push(bdef);
            }
            }
        }
        damn
    }
    pub fn create_liquid(&mut self) {
        let psystemdef = ParticleSystemDef {
            radius: 0.3,
            ..Default::default()
        };
	    let psystem: ParticleSystem = self.world.create_particle_system(&psystemdef);
        
        let mut pdef: ParticleDef = ParticleDef {
            color: ParticleColor::new(0,25,255,50),
            flags: WATER_PARTICLE|TENSILE_PARTICLE,
            //group: Option<ParticleGroup>
            //lifetime: f32
            //velocity: Vec2
            position: Vec2::new(50.0,50.0), 
            ..Default::default()
        };
        for x in 1..2000 {
            let fx = x as f32;
            pdef.position = Vec2::new(65.0+(fx%15.0),fx/50.0);
            psystem.create_particle(&pdef);
        }
    }

}
/*
		/// Water particle.
		const WATER_PARTICLE = 0,

		/// Removed after next simulation step.
		const ZOMBIE_PARTICLE = 1 << 1,

		/// Zero velocity.
		const WALL_PARTICLE = 1 << 2,
		/// With restitution from stretching.
		const SPRING_PARTICLE = 1 << 3,
		/// With restitution from deformation.
		const ELASTIC_PARTICLE = 1 << 4,
		/// With viscosity.
		const VISCOUS_PARTICLE = 1 << 5,
		/// Without isotropic pressure.
		const POWDER_PARTICLE = 1 << 6,
		/// With surface tension.
		const TENSILE_PARTICLE = 1 << 7,
		/// Mix color between contacting particles.
		const COLOR_MIXING_PARTICLE = 1 << 8,
		/// Call b2DestructionListener on destruction.
		const DESTRUCTION_LISTENER_PARTICLE = 1 << 9,
		/// Prevents other particles from leaking.
		const BARRIER_PARTICLE = 1 << 10,
		/// Less compressibility.
		const STATIC_PRESSURE_PARTICLE = 1 << 11,
		/// Makes pairs or triads with other particles.
		const REACTIVE_PARTICLE = 1 << 12,
		/// With high repulsive force.
		const REPULSIVE_PARTICLE = 1 << 13,
		/// Call b2ContactListener when this particle is about to interact with
		/// a rigid body or stops interacting with a rigid body.
		/// This results in an expensive operation compared to using
		/// b2_fixtureContactFilterParticle to detect collisions between
		/// particles.
		const FIXTURE_CONTACT_LISTENER_PARTICLE = 1 << 14,
		/// Call b2ContactListener when this particle is about to interact with
		/// another particle or stops interacting with another particle.
		/// This results in an expensive operation compared to using
		/// b2_particleContactFilterParticle to detect collisions between
		/// particles.
		const PARTICLE_CONTACT_LISTENER_PARTICLE = 1 << 15,
		/// Call b2ContactFilter when this particle interacts with rigid bodies.
		const FIXTURE_CONTACT_FILTER_PARTICLE = 1 << 16,
		/// Call b2ContactFilter when this particle interacts with other
		/// particles.
		const PARTICLE_CONTACT_FILTER_PARTICLE = 1 << 17,
        */

