mod world_particle;
mod world;

use std::cmp::min;
use rand::Rng;
use raylib::prelude::*;
use raylib::consts::MouseButton::*;

const WIDTH: usize = 300;
const HEIGHT: usize = 300;

use world_particle::*;
use world::*;


fn main() {
    let mut world: Chunk = Chunk::new(Vector2::new(WIDTH as f32, HEIGHT as f32));

    let (mut rl, thread) = raylib::init()
        .size(WIDTH as i32, HEIGHT as i32)
        .title("Liquid Simulation")
        .build();

    while !rl.window_should_close() {
        if (rl.is_mouse_button_down(MOUSE_BUTTON_LEFT)) {
            world.place(rl.get_mouse_position(), ParticleEnum::Liquid(Liquid::new()));
        }
        else if (rl.is_mouse_button_down(MOUSE_BUTTON_RIGHT)) {
            world.place(rl.get_mouse_position(), ParticleEnum::Powder(Powder::new()));
        }
        world.update();

        // Render
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        world.render(&mut d);
    }
}
