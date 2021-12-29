mod particle;
use std::{collections::VecDeque, time::Instant};

use particle::*;

use array2d::Array2D;

use rand::prelude::*;
use rayon::prelude::*;

use crate::{HEIGHT, WIDTH};

const SCALE: u8 = 128;

// focal length parameter
const FOCAL: f32 = 1.1;
pub struct World {
    particles: VecDeque<Particle>,
    dead: usize,

    //buffer: Array2D<[u8; 4]>,

    frame: u64,
    pub time: f32, //time in milliseconds
    pub decay_time: f32,
    pub plotting_time: f32,
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    pub fn new() -> Self {
        Self {
            particles: VecDeque::with_capacity(4096),
            dead: 0,

            //buffer: Array2D::filled_with([0, 0, 0, 0], HEIGHT as usize, WIDTH as usize),

            frame: 0,
            time: 0.0,
            decay_time: 0.0,
            plotting_time: 0.0,
        }
    }

    pub fn count(&self) -> usize {
        self.particles.len()
    }

    pub fn avg_rate(&self) -> f32 {
        self.frame as f32 / self.time
    }



    pub fn emit_particles(&mut self, num: u16) {
        for _ in 0..num{
            self.particles.push_front(Particle::new_random(200));
        }
        // for i in 0..self.particles.len().saturating_sub(NUMBER) {
        //     self.particles.pop_front();
        //     //print!("{}", if self.particles.pop_front().unwrap().live {"L"} else {"D"});
        // }

        // println!();
    }

    pub fn update(&mut self) {
        self.frame += 1;
        if self.frame % 1 == 0 {
            self.emit_particles(10);
        }

        if self.frame % 10 == 0 {
            // self.particles.retain(|p| p.life > 0);
            // self.particles.par_iter().enumerate().for_each(|(ix, p)| if p.life == 0 {self.particles.swap_remove_back(ix);});

            // let dead: Vec<usize> = self
            //     .particles
            //     .par_iter()
            //     .enumerate()
            //     .filter_map(|(ix, p)| if p.life == 0 { Some(ix) } else { None })
            //     .collect();
            //self.dead = 0;

            // if let Some(ix) = self.particles.iter().rev().position(|p|p.life == -1) {
            //     print!("first dead: {} ", ix)
            //     //self.particles.swap_remove_back(ix);
            // }

            // if let Some(ix) = self.particles.iter().rev().position(|p|p.life > 0) {
            // self.dead -= ix;

            // println!(
            //     "dead: {} ({}%)",
            //     self.dead,
            //     self.dead * 100 / (self.particles.len() + 1)
            // );
            self.particles.truncate(self.particles.len() - self.dead);
            self.dead = 0;
        }

        let mut rng = thread_rng();
        let mut new_dead = 0;
        let new_particles = self
            .particles
            .iter_mut()
            .filter_map(|p| {
                p.x += p.vx as i32; //-(SCALE as i32) + ...
                p.y += p.vy as i32;
                p.z += TREE_H / (10 + p.z);
                let mut particle = None;
                if p.life > 0 {
                    p.vx = rng.gen::<i8>() / 4; //rng.gen_range(-128..=2);
                    p.vy = rng.gen::<i8>() / 4; //rng.gen_range(-2..=2);

                    // if p.x < 0 {
                    //     p.x += SCALE as i32 * WIDTH as i32;
                    // }
                    // if p.x > SCALE as i32 * WIDTH as i32 {
                    //     p.x -= SCALE as i32 * WIDTH as i32;
                    // }
                    // if p.y < 0 {
                    //     p.y += SCALE as i32 * HEIGHT as i32;
                    // }
                    // if p.y > SCALE as i32 * HEIGHT as i32 {
                    //     p.y -= SCALE as i32 * HEIGHT as i32;
                    // }

                    // if rng.gen_ratio(p.z as u32, TREE_H as u32 + p.life.max(0) as u32 + 3000) {
                    if rng.gen_ratio(p.z as u32, 20*TREE_H as u32){                        
                        let mut new_particle = p.fork();
                        new_particle.vx = rng.gen(); //rng.gen_range(-10..=10);
                        new_particle.vy = rng.gen(); //rng.gen_range(-10..=10);
                        particle = Some(new_particle);
                    }
                    p.life -= 1;
                } else {
                    if p.life == 0 {
                        new_dead += 1;
                        p.life -= 1;
                    }
                    p.vx = 0;
                    p.vy = 0;
                }
                particle
            })
            .collect::<Vec<_>>();
        // self.particles.append(&mut new_particles);
        for new_particle in new_particles {
            self.particles.push_front(new_particle) // my current 'cleaning' system relies on the fact that newest particles are in front
        }
        self.dead += new_dead;
    }

    pub fn draw(&mut self, frame: &mut [u8]) {
        let mut instant = Instant::now();

        // here rayon actually gave me speed up from 9-12 ms to 2-3ms
        frame.par_chunks_exact_mut(4).for_each(|p| {
            let pix: Vec<u8> = p
                .iter_mut()
                .map(|pix| pix.saturating_sub(1)) //(*pix as f32 * 0.999999) as u8)
                .collect(); //decay ef
            p.clone_from_slice(&pix)
        });

        self.decay_time += instant.elapsed().as_secs_f32();
        let decay = self.decay_time / self.frame as f32;
        print!("decay: {:.3}ms ", decay * 1000.0);
        instant = Instant::now();

        // horizontal scrolling
        // todo test performance with rayon
        // for line in frame.chunks_exact_mut(4 * WIDTH as usize) {
        //     line.rotate_left(4)
        // }

        //vertical scrolling
        // need to use Array2D for that...

        // todo averaging
        // let scrolling = instant.elapsed().as_secs_f32();
        // print!(
        //     "scrolling: {:.3}ms ({:.3}) ",
        //     scrolling * 1000.0,
        //     scrolling / decay
        // );
        //instant = Instant::now();

        for p in self.particles.iter() {
            //variable perspective projection factor, 1.0 means infinite 'focus' distance
            //let t = (3.0 + (0.1 * self.time * std::f32::consts::TAU).sin())/2.0;

            let t = FOCAL;

            let cx = p.x as f32 / SCALE as f32 - WIDTH as f32 / 2.0;
            let cy = p.y as f32 / SCALE as f32 - HEIGHT as f32 / 2.0;

            // orthogonal variant
            // let x = p.x / SCALE as i32;
            // let y = p.y / SCALE as i32;

            // linear interpolation, h is tree height, H is its maximum height, a is "focal length" parameter
            // a * (H - h)/H + 1.0 * (h)/H = (aH - ah + h)/H = (a - (a - 1)h/H)

            //perspective
            let k = (t - (t - 1.0) * p.z as f32 / 255.0).max(1.00);
            let x = WIDTH as i32 / 2 + (cx / k) as i32;
            let y = HEIGHT as i32 / 2 + (cy / k) as i32;
            // println!("p.z = {:.3}, {:.3}", p.z,  (self.time - (self.time - 1.0) * p.z as f32 / 255.0));
            // let z = p.z / SCALE as i32;

            let ind = 4 * (y * crate::WIDTH as i32 + x) as usize;

            if x > 0 && x < WIDTH as i32 && y > 0 && y < HEIGHT as i32{
                frame.get_mut(ind..ind + 4).map(|pix| {
                    pix.copy_from_slice(&[
                        p.r,
                        (p.g / 2).saturating_add((p.life / 2) as u8),
                        (p.b as f32 * p.life as f32 / LIFE as f32) as u8,
                        0xff,
                    ])
                });
            }

            // if let Some(pixel) = frame
            //     .chunks_exact_mut(4)
            //     .nth((y * crate::WIDTH as i32 + x) as usize)
            // {
            //     pixel.copy_from_slice(&[
            //         p.r,
            //         (p.g / 2).saturating_add((p.life / 2) as u8),
            //         (p.b as f32 * p.life as f32 / LIFE as f32) as u8,
            //         0xff,
            //     ]);
            // }
        }

        let delta = instant.elapsed().as_secs_f32();
        self.plotting_time += delta;
        let plotting = self.plotting_time / self.frame as f32;
        print!(
            "plotting: {:.3}ms ({:.3} ns per particle) ",
            plotting * 1000.0,
            delta * 1000_000_000.0 / self.count() as f32
        );
    }
}
