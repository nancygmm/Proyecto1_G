mod framebuffer;
mod maze;
mod player;
mod caster;

use minifb::{ Window, WindowOptions, Key };
use nalgebra_glm::{Vec2};
use std::f32::consts::PI;
use std::time::Duration;
use crate::framebuffer::Framebuffer;
use crate::maze::load_maze;
use crate::player::{Player};
use crate::caster::{cast_ray};

enum RenderMode {
    Mode2D,
    Mode3D,
}

fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize, cell: char) {
    if cell == ' ' {
        return;
    }

    framebuffer.set_current_color(0xFFDDDD);

    for x in xo..xo + block_size {
        for y in yo..yo + block_size {
            framebuffer.point(x, y);
        }
    }
}

fn render_2d(framebuffer: &mut Framebuffer, player: &Player) {
    let maze = load_maze("./maze.txt");
    let block_size = 100;

    // Draws maze
    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            draw_cell(framebuffer, col * block_size, row * block_size, block_size, maze[row][col]);
        }
    }

    // Draw player
    framebuffer.set_current_color(0xFFDDD);
    framebuffer.point(player.pos.x as usize, player.pos.y as usize);

    // Cast ray (if needed for 2D view)
    cast_ray(&maze, &player, player.a, block_size, 1000.0);
}

fn render_3d(framebuffer: &mut Framebuffer, player: &Player) {
    let maze = load_maze("./maze.txt");
    let block_size = 100;

    // Set the player's field of view and other raycasting parameters
    let fov = PI / 3.0; // 60 degrees
    let num_rays = framebuffer.width;
    let max_depth = 1000.0;

    for ray_index in 0..num_rays {
        let ray_angle = player.a - (fov / 2.0) + (ray_index as f32 / num_rays as f32) * fov;

        if let Some((distance, _hit_x, _hit_y)) = cast_ray(&maze, &player, ray_angle, block_size, max_depth) {
            let distance = distance * (PI / 3.0).cos(); // Correcting fish-eye effect
            let wall_height = (framebuffer.height as f32 / distance) as usize;
            let wall_start = framebuffer.height / 2 - wall_height / 2;
            let wall_end = wall_start + wall_height;

            framebuffer.set_current_color(0xFFDDDD);

            for y in wall_start..wall_end {
                framebuffer.point(ray_index, y);
            }
        }
    }
}

fn main() {
    let window_width = 1100;
    let window_height = 700;
    let framebuffer_width = 1300;
    let framebuffer_height = 900;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Proyecto 1",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    framebuffer.set_background_color(0x333355);

    let player = Player {
        pos: Vec2::new(150.0, 150.0),
        a: PI / 3.0,
    };

    let mut render_mode = RenderMode::Mode2D;
    let mut last_m_key_state = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let m_key_state = window.is_key_down(Key::M);

        // Toggle render mode if M is pressed
        if m_key_state && !last_m_key_state {
            render_mode = match render_mode {
                RenderMode::Mode2D => RenderMode::Mode3D,
                RenderMode::Mode3D => RenderMode::Mode2D,
            };
        }
        last_m_key_state = m_key_state;

        framebuffer.clear();
        
        match render_mode {
            RenderMode::Mode2D => render_2d(&mut framebuffer, &player),
            RenderMode::Mode3D => render_3d(&mut framebuffer, &player),
        }

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
