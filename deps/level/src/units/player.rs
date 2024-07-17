use std::ops::{Deref, DerefMut};

use gm::flat::{Point, Shape};
use refs::Own;

use crate::{LevelManager, Sprite, SpriteData, Unit, Weapon};

pub struct Player {
    pub unit:   Own<Unit>,
    pub weapon: Own<Weapon>,
}

impl Sprite for Player {
    fn update(&mut self) {
        let cursor = LevelManager::level().cursor_position;
        self.weapon.rotation = self.position().angle_to(cursor);
        self.weapon.position = self.unit.position();
        self.weapon.velocity = self.unit.body.velocity();

        // if !self.image.is_empty() {
        //     self.image().flipped = cursor.x < self.position().x;
        // }
        // if self.weapon.image.is_ok() {
        //     self.weapon.image().flipped_y = cursor.x < self.position().x;
        // }
    }

    fn position(&self) -> Point {
        self.unit.position()
    }

    fn rotation(&self) -> f32 {
        self.unit.rotation()
    }
    //
    // fn draw(&self) {
    //     self.unit.draw();
    //     self.weapon.draw();
    // }

    fn make(shape: Shape, position: Point) -> Own<Self>
    where Self: Sized {
        Own::new(Player {
            unit:   Unit::make(shape.clone(), position),
            weapon: Weapon::make(shape, position),
        })
    }
}

impl Deref for Player {
    type Target = SpriteData;

    fn deref(&self) -> &Self::Target {
        &self.unit.body
    }
}

impl DerefMut for Player {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.unit.body
    }
}
