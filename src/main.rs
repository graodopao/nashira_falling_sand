use std::cmp::min;
use rand::Rng;
use raylib::prelude::*;

const WIDTH: usize = 300;
const HEIGHT: usize = 300;

fn main() {

    let mut world: Vec<Vec<bool>> = Vec::new();
    let mut count = 0;
    loop
    {
        if count >= HEIGHT {break;}

        // Generates random particles

        let choice = rand::thread_rng().gen_range(0..3);
        if choice == 0
        {
            world.push(vec![true; WIDTH]);

            let mut _w = 0;
            loop
            {
                if _w >= WIDTH {break;}

                if rand::thread_rng().gen_range(0..2) == 1
                {
                    world[count][_w] = false;
                }

                _w += 1;
            }
        }
        else
        {
            world.push(vec![false; WIDTH]);
        }

        count += 1;
    }


    let (mut rl, thread) = raylib::init()
        .size(WIDTH as i32, HEIGHT as i32)
        .title("Liquid Simulation")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        render_world(&mut world, &mut d);
    }

}

fn render_world(world: &mut Vec<Vec<bool>>, handle: &mut RaylibDrawHandle) {
    // World settings

    let mut choice = 0;
    let mut count = 0;
    // Will store the final string
    let mut final_string = String::new();    

    // Analyzes the world and makes the particles to react
    for y in 0..world.len() - 1
    {
        // Y row
        for x in 0..world[y].len() - 1
        {

            // Updates the particles from bottom to top
            let mut y_final_value = HEIGHT - 1 - y;
            // X row
            if world[y_final_value][x] == true && y_final_value < world.len() - 1
            {
                // Makes it fall
                world[y_final_value][x] = false;

                if world[y_final_value + 1][x] == true
                {
                    // If there's already a particle under this one, move to the laterals
                    if x != 0 && world[y_final_value + 1][x - 1] != true && y_final_value < world.len() - 1
                    {
                        world[y_final_value + 1][x - 1] = true;
                    }
                    else if x != WIDTH - 1 && world[y_final_value + 1][x + 1] != true && y_final_value < world.len() - 1
                    {
                        world[y_final_value + 1][x + 1] = true;
                    }
                    else
                    {
                        // If there's no way to move downwards, only moves sideways if possible
                        choice = rand::thread_rng().gen_range(0..2);

                        if x > 0 && world[y_final_value][x - 1] != true && choice == 0
                        {
                            world[y_final_value][x - 1] = true;
                        }
                        else
                        {
                            if x < WIDTH - 1 && world[y_final_value][x + 1] != true && choice == 1
                            {
                                world[y_final_value][x + 1] = true;
                            }
                            else
                            {
                                world[y_final_value][x] = true;
                            }
                        }
                    }
                }
                else
                {
                    world[y_final_value + 1][x] = true;
                }
            }
        }
        final_string.push('\n');
    }

    // Gets information about the world to display it
    final_string = String::from("");

    for y in 0..(HEIGHT)
    {
        for x in 0..(WIDTH)
        {
            //final_string.push(world[y][x]);
            handle.draw_pixel(x as i32, y as i32, if world[y][x] { Color::BLACK } else { Color::WHITE });
        }
        //final_string.push('\n');
    }
}

