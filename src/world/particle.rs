use crate::{HEIGHT, WIDTH};
use rand::prelude::*;

use crate::world::SCALE;
use crate::world::FOCAL;

pub const LIFE: i32 = 200;
pub const TREE_H: i32 = 255;

pub struct Particle {
    pub x: i32,
    pub y: i32,
    pub z: i32,

    pub vx: i8,
    pub vy: i8,
    pub vz: i8,

    pub px: usize,
    pub py: usize,
    pub pind: usize,

    pub r: u8,
    pub g: u8,
    pub b: u8,

    pub life: i32,
}

impl Particle {
    pub fn new_random(brightness: u8) -> Self {
        let mut rng = ThreadRng::default();


        // let cx = p.x as f32 / SCALE as f32 - WIDTH as f32 / 2.0;
        // let cy = p.y as f32 / SCALE as f32 - HEIGHT as f32 / 2.0;

        // let t = FOCAL;

        // let k = (t - (t - 1.0) * p.z as f32 / TREE_H as f32).max(1.00);

        // let x = WIDTH as i32 / 2 + (cx / k) as i32;
        // let y = HEIGHT as i32 / 2 + (cy / k) as i32;

        // x max (p.z = 0): k = FOCAL; x = WIDTH/2 + WIDTH/2/FOCAL = WIDTH/2(1 + 1/FOCAL)
        // x min (p.z = 0): k = FOCAL; x = WIDTH/2 + (-WIDTH)/2/FOCAL = WIDTH/2(1 - 1/FOCAL)


        //start from end projection of the particle
        let px = rng.gen_range(0..WIDTH);
        let py = rng.gen_range(0..HEIGHT);

        let cx = (px as i32 - WIDTH as i32/2) as f32*FOCAL;
        let cy = (py as i32 - HEIGHT as i32/2) as f32*FOCAL;

        let x = (cx + WIDTH as f32/2.0) as i32 * SCALE as i32;
        let y = (cy + HEIGHT as f32/2.0) as i32 * SCALE as i32;

        Particle {
            // x: rng.gen_range(0..(SCALE as f32 * WIDTH as f32) as i32),
            // y: rng.gen_range(0..(SCALE as f32 * HEIGHT as f32) as i32),
            x,
            y,
            px,
            py,
            pind: py*WIDTH*4 + 4*px,
            z: 1,
            vx: 0,
            vy: 0,
            vz: 1,
            r: rng.gen_range(0x00..brightness),
            g: rng.gen_range(0x00..brightness),
            b: brightness,
            life: LIFE,
        }
    }

    pub fn fork(&self) -> Self {
        Particle {
            life: (self.life - 1).min(LIFE/2),
            z: self.z + 1,
            ..*self
        }
    }
}
