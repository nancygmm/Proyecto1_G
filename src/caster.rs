use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::player::Player;

pub fn load_maze(filename: &str) -> Vec<Vec<char>> {
    let file = File::open(filename).unwrap();

    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

pub fn cast_ray(maze: &Vec<Vec<char>>, player: &Player, angle: f32, block_size: usize, max_depth: f32) -> Option<(f32, f32, f32)> {
    let mut distance = 0.0;
    let step_size = 0.1;
    
    while distance < max_depth {
        let test_x = player.pos.x + distance * angle.cos();
        let test_y = player.pos.y + distance * angle.sin();

        if maze[test_y as usize / block_size][test_x as usize / block_size] != ' ' {
            return Some((distance, test_x, test_y));
        }

        distance += step_size;
    }

    None
}
