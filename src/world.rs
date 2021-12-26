mod particle;
use std::collections::VecDeque;

use particle::*;

use rand::prelude::*;
use rayon::prelude::*;

use crate::{HEIGHT, WIDTH};

const NUMBER: usize = 8192;
const SCALE: u8 = 16;
pub struct World {
    particles: VecDeque<Particle>,
    frame: u64,
    pub time: f32, //time in milliseconds
    updated: bool,
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    pub fn new() -> Self {
        Self {
            particles: VecDeque::with_capacity(4096),
            frame: 0,
            time: 0.0,
            updated: false,
        }
    }

    pub fn count(&self) -> usize {
        self.particles.len()
    }

    pub fn avg_rate(&self) -> f32 {
        self.frame as f32 / self.time
    }

    pub fn emit_particles(&mut self) {
        self.particles.push_back(Particle::new_random(255));
        // for i in 0..self.particles.len().saturating_sub(NUMBER) {
        //     self.particles.pop_front();
        //     //print!("{}", if self.particles.pop_front().unwrap().live {"L"} else {"D"});
        // }

        self.particles.retain(|p| p.life > 0);

        // println!();
    }

    pub fn update(&mut self) {
        self.frame += 1;
        if self.frame % 10 == 0 {
            self.emit_particles()
        }
        if !self.updated {
            let mut rng = thread_rng();
            let new_particles = self
                .particles
                .iter_mut()
                .filter_map(|p| {
                    p.x += p.vx as i32;
                    p.y += p.vy as i32;
                    let mut particle = None;
                    if p.life > 0 {
                        p.life -= 1;
                        p.vx = rng.gen::<i8>()/4; //rng.gen_range(-128..=2);
                        p.vy = rng.gen::<i8>()/4; //rng.gen_range(-2..=2);
                        if p.x < -(SCALE as i32) * (WIDTH as i32) / 3
                            || p.x > (SCALE as i32) * WIDTH as i32 * 4 / 3
                            || p.y < -(SCALE as i32) * (HEIGHT as i32) / 3
                            || p.y > (SCALE as i32) * HEIGHT as i32 * 4 / 3
                        {
                            p.life = 0;
                            p.r = 250;
                        } else if rng.gen_ratio(1, 30) {
                            let mut new_particle = p.fork();
                            new_particle.vx = rng.gen(); //rng.gen_range(-10..=10);
                            new_particle.vy = rng.gen(); //rng.gen_range(-10..=10);
                            particle = Some(new_particle);
                        }
                    } else {
                        p.vx = 0;
                        p.vy = 0;
                    }
                    particle
                })
                .collect::<Vec<_>>();
            // self.particles.append(&mut new_particles);
            self.particles.par_extend(new_particles);
        }
    }

    pub fn draw(&self, frame: &mut [u8]) {
        for p in frame.chunks_exact_mut(4) {
            let pix: Vec<u8> = p
                .iter_mut()
                .map(|pix| (*pix as f32 * 0.9997) as u8)
                .collect(); //decay ef
            p.clone_from_slice(&pix)
        }

        for p in self.particles.iter() {
            let x = p.x / SCALE as i32;
            let y = p.y / SCALE as i32;
            let z = p.z / SCALE as i32;
            let mut c = [0, 0, 0, 0];

            if let Some(pixel) = frame
                .chunks_exact_mut(4)
                .nth((y * crate::WIDTH as i32 + x) as usize)
            {
                pixel.copy_from_slice(&[p.r, (p.g/2).saturating_add((p.life/2) as u8), (p.b as f32*(p.life as f32 / 255.0)) as u8, 0xff]);
            }
        }
    }
}
