use std::ops::{Deref, DerefMut};

use gm::flat::{Point, Shape};
use rapier2d::{
    dynamics::{CoefficientCombineRule, RigidBodyHandle},
    geometry::ColliderHandle,
};
use refs::Own;

use crate::{Body, Sprite, SpriteData};

pub struct Unit {
    pub body: Own<Body>,
}

impl Sprite for Unit {
    fn update(&mut self) {
        self.body.update();
    }

    fn make(shape: Shape, position: Point) -> Own<Self>
    where Self: Sized {
        let mut body = Body::make(shape, position);

        body.lock_rotations();
        body.collider_mut().set_restitution(0.0);
        body.collider_mut().set_restitution_combine_rule(CoefficientCombineRule::Min);

        Own::new(Unit { body })
    }

    fn rigid_handle(&self) -> Option<RigidBodyHandle> {
        self.body.rigid_handle()
    }

    fn collider_handle(&self) -> Option<ColliderHandle> {
        self.body.collider_handle()
    }
}

impl Deref for Unit {
    type Target = SpriteData;
    fn deref(&self) -> &SpriteData {
        &self.body
    }
}

impl DerefMut for Unit {
    fn deref_mut(&mut self) -> &mut SpriteData {
        &mut self.body
    }
}
