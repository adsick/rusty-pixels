use crate::{HEIGHT, WIDTH};
use rand::prelude::*;

use crate::world::SCALE;

pub struct Particle {
    pub x: i32,
    pub y: i32,
    pub z: i32,

    pub vx: i8,
    pub vy: i8,
    pub vz: i8,

    pub r: u8,
    pub g: u8,
    pub b: u8,

    pub life: i32,
}

impl Particle {
    pub fn new_random(brightness: u8) -> Self {
        let mut rng = ThreadRng::default();

        Particle {
            x: rng.gen_range(0..SCALE as i32 * WIDTH as i32),
            y: rng.gen_range(0..SCALE as i32 * HEIGHT as i32),
            z: 0,
            vx: 0,
            vy: 0,
            vz: 0,
            r: rng.gen_range(0x00..brightness),
            g: rng.gen_range(0x00..brightness),
            b: brightness,
            life: 255,
        }
    }

    pub fn fork(&self) -> Self {
        Particle {
            life: (self.life + 11).min(100),
            ..*self
        }
    }
}
