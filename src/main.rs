mod framebuffer;
mod maze;
mod player;
mod caster;

use std::io::BufReader;
use minifb::{Window, WindowOptions, Key};
use nalgebra_glm::Vec2;
use std::f32::consts::PI;
use std::time::Duration;
use std::fs::File;
use rodio::{Decoder, OutputStream, Sink};
use std::thread;
use gilrs::{Gilrs, Event, Button, Axis};
use crate::framebuffer::Framebuffer;
use crate::maze::load_maze;
use crate::player::Player;
use crate::caster::cast_ray;

enum RenderMode {
    Mode2D,
    Mode3D,
}

fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize, cell: char) {
    if cell == ' ' {
        return;
    }

    framebuffer.set_current_color(0x000000);

    for x in xo..xo + block_size {
        for y in yo..yo + block_size {
            framebuffer.point(x, y);
        }
    }
}

fn render_2d(framebuffer: &mut Framebuffer, player: &Player) {
    let maze = load_maze("./maze.txt");
    let block_size = 100;

    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            draw_cell(framebuffer, col * block_size, row * block_size, block_size, maze[row][col]);
        }
    }

    framebuffer.set_current_color(0xFFFFFF);
    let player_size = 5;
    for x in player.pos.x as usize - player_size..=player.pos.x as usize + player_size {
        for y in player.pos.y as usize - player_size..=player.pos.y as usize + player_size {
            framebuffer.point(x, y);
        }
    }
}

fn render_3d(framebuffer: &mut Framebuffer, player: &mut Player) {
    let maze = load_maze("./maze.txt");
    let block_size = 100;

    let fov = PI / 3.0; 
    let num_rays = framebuffer.width;
    let max_depth = 1000.0;

    for ray_index in 0..num_rays {
        let ray_angle = player.a - (fov / 2.0) + (ray_index as f32 / num_rays as f32) * fov;

        if let Some((distance, _hit_x, _hit_y)) = cast_ray(&maze, player, ray_angle, block_size, max_depth) {
            let distance = distance * (fov / 2.0).cos(); 
            let wall_height = (framebuffer.height as f32 / distance * 6.0) as usize; 
            let wall_start = framebuffer.height / 2 - wall_height / 2;
            let wall_end = wall_start + wall_height;

            framebuffer.set_current_color(0x000000);

            for y in wall_start..wall_end {
                framebuffer.point(ray_index, y);
            }

            if distance < 5.0 {
                player.pos.x = player.pos.x; 
                player.pos.y = player.pos.y;
            }
        }
    }
}

fn render_minimap(
    framebuffer: &mut Framebuffer,
    player: &Player,
    x_offset: usize,
    y_offset: usize,
    scale: f32,
) {
    let maze = load_maze("./maze.txt");
    let block_size = (100.0 * scale) as usize;

    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            draw_cell(framebuffer, x_offset + col * block_size, y_offset + row * block_size, block_size, maze[row][col])
        }
    }

    framebuffer.set_current_color(0xFFDDD);
    let player_x = x_offset + (player.pos.x * scale) as usize;
    let player_y = y_offset + (player.pos.y * scale) as usize;
    if player_x < framebuffer.width && player_y < framebuffer.height {
        framebuffer.point(player_x, player_y);
    }

    let num_rays = 5;
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);

        cast_ray(&maze, player, a, 100, 1000.0); 
    }
}

fn main() {
    let audio_thread = thread::spawn(|| {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        let file = File::open("./Style.wav").unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap();

        sink.append(source);
        sink.sleep_until_end();
    });

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

    framebuffer.set_background_image("./Liso.webp");
    framebuffer.clear();

    let mut player = Player {
        pos: Vec2::new(150.0, 150.0),
        a: PI / 3.0,
        fov: PI / 3.0, 
    };

    let mut render_mode = RenderMode::Mode2D;
    let mut last_m_key_state = false;
    let rotation_speed = 0.1;
    let movement_speed = 5.0;
    let block_size = 100;

    let mut previous_mouse_x = window.get_mouse_pos(minifb::MouseMode::Pass).unwrap_or((0.0, 0.0)).0;

    // Inicializar gilrs
    let mut gilrs = Gilrs::new().unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let m_key_state = window.is_key_down(Key::M);

        if m_key_state && !last_m_key_state {
            render_mode = match render_mode {
                RenderMode::Mode2D => RenderMode::Mode3D,
                RenderMode::Mode3D => RenderMode::Mode2D,
            };
        }
        last_m_key_state = m_key_state;

        // Manejar entradas de teclado
        if window.is_key_down(Key::Left) || window.is_key_down(Key::A) {
            player.rotate_left(rotation_speed);
        }
        if window.is_key_down(Key::Right) || window.is_key_down(Key::D) {
            player.rotate_right(rotation_speed);
        }

        if let Some((mouse_x, _)) = window.get_mouse_pos(minifb::MouseMode::Pass) {
            let delta_x = mouse_x - previous_mouse_x;
            player.a += delta_x * 0.005; 
            previous_mouse_x = mouse_x;
        }

        if window.is_key_down(Key::Up) || window.is_key_down(Key::W) {
            player.move_forward(&load_maze("./maze.txt"), block_size, movement_speed);
        }
        if window.is_key_down(Key::Down) || window.is_key_down(Key::S) {
            player.move_backward(&load_maze("./maze.txt"), block_size, movement_speed);
        }

        // Manejar entradas del controlador
        while let Some(Event { id, event, time }) = gilrs.next_event() {
            match event {
                gilrs::EventType::AxisChanged(Axis::LeftStickX, value, _) => {
                    if value < -0.5 {
                        player.rotate_left(rotation_speed);
                    } else if value > 0.5 {
                        player.rotate_right(rotation_speed);
                    }
                }
                gilrs::EventType::AxisChanged(Axis::LeftStickY, value, _) => {
                    if value < -0.5 {
                        player.move_backward(&load_maze("./maze.txt"), block_size, movement_speed);
                    } else if value > 0.5 {
                        player.move_forward(&load_maze("./maze.txt"), block_size, movement_speed);
                    }
                }
                gilrs::EventType::ButtonPressed(Button::South, _) => {
                }
                gilrs::EventType::ButtonPressed(Button::East, _) => {
                }
                _ => {}
            }
        }

        framebuffer.clear();

        match render_mode {
            RenderMode::Mode2D => render_2d(&mut framebuffer, &player),
            RenderMode::Mode3D => {
                render_3d(&mut framebuffer, &mut player);
                render_minimap(
                    &mut framebuffer,
                    &player,
                    framebuffer_width - (framebuffer_width as f32 * 0.1) as usize - 10,
                    10,
                    0.1
                );
            }
        }

        window.update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height).unwrap();
        std::thread::sleep(frame_delay);
    }

    audio_thread.join().unwrap();
}
