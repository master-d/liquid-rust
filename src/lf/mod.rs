use liquidfun::box2d::collision::shapes::polygon_shape::*;
use liquidfun::box2d::common::math::*;
use liquidfun::box2d::dynamics::body::*;
use liquidfun::box2d::dynamics::fixture::*;
use liquidfun::box2d::dynamics::world::*;
use liquidfun::box2d::collision::shapes;

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
            pos: (0.0,0.0), w: 2.0, h: 2.0, density: 1.0, 
            friction: 0.3, restitution: 0.1, dynamic: true,
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
    pub fn create_ground(&mut self, width: f32) ->BoxDef {
        let mut gdef = BoxDef { 
            pos: (1.0,1.0), 
            w: width, 
            h: 0.2, 
            color: (0,255,0),
            dynamic: false, ..Default::default() 
        };
        let ground = self.create_body(&gdef);
        gdef.set_body(ground);
        gdef
    }

}

