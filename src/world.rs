use std::mem;
use raylib::prelude::*;

use crate::HEIGHT;
use crate::world_particle::*;
use std::mem::swap;

use rand::Rng;

pub struct Chunk {
    grid: Vec<Vec<ParticleEnum>>,
    size: Vector2,
}

impl Chunk {
    pub fn new(size: Vector2) -> Chunk {
        let mut grid: Vec<Vec<ParticleEnum>> = Vec::new();

        grid.resize(size.x as usize, Vec::new());
        for x in 0..size.y as usize {
            grid[x] = Vec::new();
            grid[x].resize(size.y as usize, ParticleEnum::Void(Void::new()));
        }

        Chunk{grid, size}
    }

    pub fn update(&mut self) {
        // Updates the particles from bottom to top
        // Analyzes the world and makes the particles to react
        for x in 0..self.size.x as usize - 1
        {
            // Y row
            for y in (0..self.size.y as usize - 1).rev()
            {


                match &self.grid[x][y] {
                    ParticleEnum::Void(_) => {}
                    ParticleEnum::Liquid(liquid) => {
                        let from = Vector2::new(x as f32, y as f32);
                        let mut to = Vector2::new(x as f32, y as f32 + 1.0);

                        // This is terrible. Definetly swiching to a better approach when working on the physics
                        if (!self.is_free(to)) {
                            to.x += 1.0;
                            if (!self.is_free(to)) {
                                to.x -= 2.0;

                                if (!self.is_free(to)) {
                                    let choice = rand::thread_rng().gen_range(-1..1);
                                    to.y -= 1.0;
                                    to.x += 1.0 + choice as f32;

                                    if (!self.is_free(to)) {
                                        to.x -= choice as f32 * 2.0;

                                        if (!self.is_free(to)) {
                                            to = from.clone();
                                        }
                                    }
                                }
                            }
                        }
                        self.move_through_grid(from, to);
                    }
                    ParticleEnum::Powder(power) => {
                        let from = Vector2::new(x as f32, y as f32);
                        let mut to = Vector2::new(x as f32, y as f32 + 1.0);

                        // This is terrible. Definetly swiching to a better approach when working on the physics
                        if (!self.is_free(to)) {
                            to.x += 1.0;
                            if (!self.is_free(to)) {
                                to.x -= 2.0;
                            }
                        }
                        self.move_through_grid(from, to);
                    }
                }
            }
        }
    }

    pub fn render(&self, handle: &mut RaylibDrawHandle) {
        for x in 0..(self.size.x as usize)
        {
            for y in 0..(self.size.y as usize)
            {
                let pixel_color = match &self.grid[x][y] {
                    ParticleEnum::Void(void) => void.color,
                    ParticleEnum::Liquid(liquid) => liquid.color,
                    ParticleEnum::Powder(power) => power.color,
                };
                handle.draw_pixel(x as i32, y as i32, pixel_color);
            }
        }
    }

    // ---- Interactions ----
    pub fn place(&mut self, at: Vector2, particle: ParticleEnum) {
        self.grid[at.x as usize][at.y as usize] = particle;
    }

    pub fn is_free(&self, at: Vector2) -> bool {
        match self.grid[at.x as usize][at.y as usize] {
            ParticleEnum::Void(_) => true,
            _ => false
        }
    }

    // I'm not sure if this will stay. I might revisit how particles work in the future
    pub fn move_through_grid(&mut self, from: Vector2, to: Vector2) {
        let particle_from = self.grid[from.x as usize][from.y as usize].clone();
        let particle_to = self.grid[to.x as usize][to.y as usize].clone();

        self.grid[from.x as usize][from.y as usize] = particle_to;
        self.grid[to.x as usize][to.y as usize] = particle_from;
    }
}