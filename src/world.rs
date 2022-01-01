use std::{collections::VecDeque, time::Instant};

mod particle;
use particle::*;

mod linebuf;
use linebuf::*;

use rand::prelude::*;
use rayon::prelude::*;

use crate::{HEIGHT, WIDTH};

const SCALE: u8 = 32;

// focal length parameter
const FOCAL: f32 = 1.0;
#[derive(Default)]
pub struct World {
    particles: VecDeque<Particle>,
    dead: usize,

    //buffer: Array2D<[u8; 4]>,
    /// line buffer
    lbuffer: LineBuf,

    frame: u64,
    pub time: f32, //time in milliseconds
    pub decay_time: f32,
    pub scrolling_time: f32,
    pub plotting_time: f32,
    pub total: f32,
}

impl World {
    pub fn new() -> Self {
        Self {
            particles: VecDeque::with_capacity(4096),
            dead: 0,

            //buffer: Array2D::filled_with([0, 0, 0, 0], HEIGHT as usize, WIDTH as usize),
            ..Default::default()
        }
    }

    pub fn count(&self) -> usize {
        self.particles.len()
    }

    pub fn avg_rate(&self) -> f32 {
        self.frame as f32 / self.time
    }

    pub fn emit_particles(&mut self, num: u16) {
        if self.frame % 1 == 0 {
            for _ in 0..num {
                self.particles.push_front(Particle::new_random(200));
            }
        }
    }

    fn clear_particles(&mut self) {
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
    }

    //this will become a separate system when I move it to ECS
    fn move_particles(&mut self){
        self
            .particles
            .iter_mut()
            .for_each(|p| {
                p.x += -(SCALE as i32) * 0 + p.vx as i32;
                p.y += (SCALE as i32) / 10 + p.vy as i32;
                p.z += TREE_H / (10 + p.z);

                let cx = p.x as f32 / SCALE as f32 - WIDTH as f32 / 2.0;
                let cy = p.y as f32 / SCALE as f32 - HEIGHT as f32 / 2.0;

                let t = FOCAL;
                let k = (t - (t - 1.0) * p.z as f32 / TREE_H as f32).max(1.00);
                let px = WIDTH as i32 / 2 + (cx / k) as i32;
                let py = HEIGHT as i32 / 2 + (cy / k) as i32;

                if (0..WIDTH as i32).contains(&px) && (0..HEIGHT as i32).contains(&py) {
                    p.px = px as usize;
                    p.py = py as usize;
                    p.pind = py as usize * 4 * WIDTH + 4 * px as usize;
                } else if p.life > 0 {
                    p.life = 0;
                }
            })
    }

    pub fn update(&mut self) {
        self.frame += 1;

        self.emit_particles(2);

        self.clear_particles();

        //self.move_particles();

        let mut rng = thread_rng();
        let mut new_dead = 0;
        let new_particles = self
            .particles
            .iter_mut()
            .filter_map(|p| {
                // code for moving particles was here in the past
                p.x += -(SCALE as i32) * 0 + p.vx as i32;
                p.y += (SCALE as i32) / 10 + p.vy as i32;
                p.z += TREE_H / (10 + p.z);

                p.x %= SCALE as i32 * WIDTH as i32;
                p.y %= SCALE as i32 * HEIGHT as i32;

                let cx = p.x as f32 / SCALE as f32 - WIDTH as f32 / 2.0;
                let cy = p.y as f32 / SCALE as f32 - HEIGHT as f32 / 2.0;

                let t = FOCAL;
                let k = (t - (t - 1.0) * p.z as f32 / TREE_H as f32).max(1.00);
                let px = WIDTH as i32 / 2 + (cx / k) as i32;
                let py = HEIGHT as i32 / 2 + (cy / k) as i32;

                if (0..WIDTH as i32).contains(&px) && (0..HEIGHT as i32).contains(&py) {
                    p.px = px as usize;
                    p.py = py as usize;
                    p.pind = py as usize * 4 * WIDTH + 4 * px as usize;
                } else if p.life > 0 {
                    p.life = 0;
                }


                
                let mut particle = None;
                if p.life > 0 {
                    p.vx = rng.gen::<i8>() / 4;
                    p.vy = rng.gen::<i8>() / 4;

                    if rng.gen_ratio(p.z as u32, 20 * TREE_H as u32) {
                        let mut new_particle = p.fork();
                        new_particle.vx = rng.gen();
                        new_particle.vy = rng.gen();
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

    fn scroll(frame: &mut [u8], x: i32, y: i32) {
        // horizontal scrolling
        if x > 0 {
            for line in frame.chunks_exact_mut(4 * WIDTH as usize) {
                line.rotate_left(4 * x as usize)
            }
        } else {
            for line in frame.chunks_exact_mut(4 * WIDTH as usize) {
                line.rotate_right(4 * x.unsigned_abs() as usize)
            }
        }

        //vertical scrolling
        if y > 0 {
            frame.rotate_right(4 * WIDTH as usize * y as usize);
        } else {
            frame.rotate_left(4 * WIDTH as usize * y.unsigned_abs() as usize);
        }
    }

    fn scroll2(&mut self, x: i32, y: i32) {
        self.lbuffer.scroll(x, y);
    }

    fn decay(frame: &mut [u8]) {
        //for some reason in this realization rayon makes it worse
        frame.iter_mut().for_each(|c| {
            *c = c.saturating_sub(1);

            // *c = (*c as f32 * 0.9999999) as u8;

            // this is an old and slow version for you to know how not to do.
            // let pix: Vec<u8> = p
            //     .iter_mut()
            //     .map(|pix| pix.saturating_sub(1)) //(*pix as f32 * 0.999999) as u8)
            //     .collect();
            // p.clone_from_slice(&pix)
        });
    }

    fn decay2(&mut self) {
        self.lbuffer.decay()
    }

    fn plot(&mut self, frame: &mut [u8]) {
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
            let k = (t - (t - 1.0) * p.z as f32 / TREE_H as f32).max(1.00);
            let x = WIDTH as i32 / 2 + (cx / k) as i32;
            let y = HEIGHT as i32 / 2 + (cy / k) as i32;

            let ind = 4 * (y * crate::WIDTH as i32 + x) as usize;

            if x > 0 && x < WIDTH as i32 && y > 0 && y < HEIGHT as i32 {
                frame.get_mut(ind..ind + 4).map(|pix| {
                    pix.copy_from_slice(&[
                        p.r,
                        (p.g / 2).saturating_add((p.life / 2) as u8),
                        (p.b as f32 * p.life as f32 / LIFE as f32) as u8,
                        0xff,
                    ])
                });
            }
        }
    }

    fn lazy_plot(&mut self, frame: &mut [u8]) {
        for p in self.particles.iter() {
            let ind = p.pind;
            frame.get_mut(ind..ind + 4).map(|pix| {
                pix.copy_from_slice(&[
                    p.r,
                    (p.g / 2).saturating_add((p.life / 2) as u8),
                    (p.b as f32 * p.life as f32 / LIFE as f32) as u8,
                    0xff,
                ])
            });
        }
    }

    fn plot2(&mut self, frame: &mut [u8]) {
        // think about parallelizing it...
        for p in self.particles.iter() {
            //variable perspective projection factor, 1.0 means infinite 'focus' distance
            let t = (3.0 + (0.1 * self.time * std::f32::consts::TAU).sin()) / 2.0;

            // let t = FOCAL;

            let cx = p.x as f32 / SCALE as f32 - WIDTH as f32 / 2.0;
            let cy = p.y as f32 / SCALE as f32 - HEIGHT as f32 / 2.0;

            // orthogonal variant
            // let x = p.x / SCALE as i32;
            // let y = p.y / SCALE as i32;

            // linear interpolation, h is tree height, H is its maximum height, a is "focal length" parameter
            // a * (H - h)/H + 1.0 * (h)/H = (aH - ah + h)/H = (a - (a - 1)h/H)

            //perspective
            let k = (t - (t - 1.0) * p.z as f32 / TREE_H as f32).max(1.00);
            let x = WIDTH as i32 / 2 + (cx / k) as i32;
            let y = HEIGHT as i32 / 2 + (cy / k) as i32;

            let r = p.r;
            let g = (p.g / 2).saturating_add((p.life / 2) as u8);
            let b = (p.b as f32 * p.life as f32 / LIFE as f32) as u8;

            // todo test with plot fast
            if (0..WIDTH as i32).contains(&x) && (0..HEIGHT as i32).contains(&y) {
                unsafe {
                    self.lbuffer.plot_rgb_fast(x as usize, y as usize, r, g, b);
                }
            }
        }

        self.lbuffer.copy_to_frame(frame)
    }

    // dc: 0.135ms      scrl: 0.107ms   plt: 0.320ms (26.081 ns pp)     total: 23.112 ns    particles: 25207, fps: 2057.85
    // dc2: 0.141ms     scrl2: 0.286ms  plt2: 0.412ms (38.173 ns pp)    total: 34.300 ns    particles: 25202, fps: 2184.50
    // dc2: 0.144ms     scrl2: 0.250ms  plt2: 0.476ms (35.311 ns pp)    total: 34.768 ns    particles: 25707, fps: 2019.81      (Vec<[...]>)
    // dc2: 0.146ms     scrl2: 0.004ms  plt2: 0.708ms (26.718 ns pp)    total: 26.863 ns    particles: 32797, fps: 1436.58      (Vec<Vec<u8>>)
    // dc2: 0.145ms     scrl2: 0.005ms  plt2: 0.560ms (30.047 ns pp)    total: 27.272 ns    particles: 25050, fps: 1752.33      safe plot
    // dc2: 0.146ms     scrl2: 0.005ms  plt2: 0.502ms (26.283 ns pp)    total: 27.261 ns    particles: 24816, fps: 1755.98      unsafe plot
    // dc2: 0.146ms     scrl2: 0.005ms  plt2: 0.502ms (19.443 ns pp)    total: 26.908 ns    particles: 25104, fps: 1756.03      unsafe plot

    pub fn draw(&mut self, frame: &mut [u8]) {
        let mut instant = Instant::now();
        let total = instant;
        // decay effect
        Self::decay(frame);

        self.decay_time += instant.elapsed().as_secs_f32();
        let decay = self.decay_time / self.frame as f32;
        print!("dc: {:.3}ms ", decay * 1000.0);
        instant = Instant::now();

        if self.frame % 10 == 0 {
            Self::scroll(frame, 0, 1);
        }

        self.scrolling_time += instant.elapsed().as_secs_f32();
        let scrolling = self.scrolling_time / self.frame as f32;
        print!("scrl: {:.3}ms ", scrolling * 1000.0);
        instant = Instant::now();

        //self.lbuffer.from_frame(frame);

        self.lazy_plot(frame);

        let delta = instant.elapsed().as_secs_f32();
        self.plotting_time += delta;
        let plotting = self.plotting_time / self.frame as f32;
        print!(
            "plt: {:.3}ms ({:.3} ns pp) ",
            plotting * 1000.0,
            delta * 1000_000_000.0 / self.count() as f32
        );

        self.total += total.elapsed().as_secs_f32();
        print!(
            "total: {:.3} ns ",
            self.total * 1000_000_000.0 / self.frame as f32 / self.count() as f32
        )
    }

    pub fn draw2(&mut self, frame: &mut [u8]) {
        let mut instant = Instant::now();
        let total = instant;
        self.decay2();

        self.decay_time += instant.elapsed().as_secs_f32();
        let decay = self.decay_time / self.frame as f32;
        print!("dc2: {:.3}ms  ", decay * 1000.0);
        instant = Instant::now();

        if self.frame % 10 == 0 {
            self.scroll2(0, 1);
        }

        self.scrolling_time += instant.elapsed().as_secs_f32();
        let scrolling2 = self.scrolling_time / self.frame as f32;
        print!("scrl2: {:.3}ms ", scrolling2 * 1000.0);
        instant = Instant::now();

        self.plot2(frame);

        let delta = instant.elapsed().as_secs_f32();
        self.plotting_time += delta;
        let plotting = self.plotting_time / self.frame as f32;
        print!(
            "plt2: {:.3}ms ({:.3} ns pp) ",
            plotting * 1000.0,
            delta * 1000_000_000.0 / self.count() as f32,
        );

        self.total += total.elapsed().as_secs_f32();
        print!(
            "total: {:.3} ns ",
            self.total * 1000_000_000.0 / self.frame as f32 / self.count() as f32
        )
    }
}
