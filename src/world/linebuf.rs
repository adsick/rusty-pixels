use rayon::prelude::*;
use rayon::slice::ParallelSliceMut;

use crate::{HEIGHT, WIDTH};

use std::slice::Iter;
use std::slice::IterMut;
// pub struct LineBuf(Vec<[u8; WIDTH as usize * 4]>);
pub struct LineBuf(Vec<Vec<u8>>);

impl Default for LineBuf {
    fn default() -> Self {
        // Self([[0; WIDTH as usize * 4]; HEIGHT as usize])
        // Self(vec![[0; WIDTH as usize * 4]; HEIGHT as usize])
        Self(vec![vec![0; WIDTH as usize * 4]; HEIGHT as usize])

    }
}

impl LineBuf {

    pub fn lines_mut(&mut self) -> IterMut<'_, Vec<u8>> {
        self.0.iter_mut()
    }

    pub fn scroll(&mut self, x: i32, y: i32) {
        // horizontal scrolling
        // todo: try with rayon
        // upd: rayon actually made perf 2x time worse here
        if x > 0 {
            for line in self.lines_mut() {
                line.rotate_left(4 * x as usize)
            }
        } else {
            for line in self.lines_mut() {
                line.rotate_right(4 * x.unsigned_abs() as usize)
            }
        }

        // vertical scrolling
        // this is 35x times better than old one
        // upd, strange, but I've lost performance after moving from Vec<Vec<u8>> to [[u8; ...]; ...]
        if y > 0 {
            self.0.rotate_right(y as usize)
        } else {
            self.0.rotate_left(y.unsigned_abs() as usize);
        }
    }

    pub fn decay(&mut self) {
        self.0
            .iter_mut()
            .flatten()
            .for_each(|c| *c = c.saturating_sub(1))
    }

    pub fn from_frame(&mut self, frame: &[u8]) {
        self.0
            .iter_mut()
            .zip(frame.chunks_exact(4 * WIDTH as usize))
            .for_each(|(dst, src)| dst.copy_from_slice(src));
    }

    pub fn plot_rgb(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8){
        let pos = 4 * x;
        self.0[y][pos..pos+4].copy_from_slice(&[r, g, b, 0xff]);
    }

    pub fn plot_rgba(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8, a: u8){
        let pos = 4 * x;
        self.0[y][pos..pos+4].copy_from_slice(&[r, g, b, a]);
    }
    
    pub unsafe fn plot_rgb_fast(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8){
        let pos = 4 * x;
            self.0.get_unchecked_mut(y).get_unchecked_mut(pos..pos+4).copy_from_slice(&[r, g, b, 0xff]);
    }

    pub unsafe fn plot_rgba_fast(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8, a: u8){
        let pos = 4 * x;
        self.0.get_unchecked_mut(y).get_unchecked_mut(pos..pos+4).copy_from_slice(&[r, g, b, a]);
    }

    /// into frame
    pub fn copy_to_frame(&self, frame: &mut [u8]){
        frame.par_chunks_exact_mut(4 * WIDTH as usize)
        .zip(self.0.par_iter())
        .for_each(|(dst, src)| dst.copy_from_slice(&src));
    }
}
