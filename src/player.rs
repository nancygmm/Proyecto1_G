use nalgebra_glm::{Vec2};

pub struct Player {
    pub pos: Vec2,
    pub a: f32, // angulo de vista
}

impl Player {
    pub fn move_forward(&mut self, maze: &Vec<Vec<char>>, block_size: usize, distance: f32) {
        let new_x = self.pos.x + distance * self.a.cos();
        let new_y = self.pos.y + distance * self.a.sin();

        if !self.is_collision(maze, new_x, new_y, block_size) {
            self.pos.x = new_x;
            self.pos.y = new_y;
        }
    }

    pub fn move_backward(&mut self, maze: &Vec<Vec<char>>, block_size: usize, distance: f32) {
        let new_x = self.pos.x - distance * self.a.cos();
        let new_y = self.pos.y - distance * self.a.sin();

        if !self.is_collision(maze, new_x, new_y, block_size) {
            self.pos.x = new_x;
            self.pos.y = new_y;
        }
    }

    fn is_collision(&self, maze: &Vec<Vec<char>>, x: f32, y: f32, block_size: usize) -> bool {
        let maze_x = (x / block_size as f32) as usize;
        let maze_y = (y / block_size as f32) as usize;

        if maze_y >= maze.len() || maze_x >= maze[0].len() {
            return true;
        }

        maze[maze_y][maze_x] != ' '
    }

    pub fn rotate_left(&mut self, angle: f32) {
        self.a -= angle;
    }

    pub fn rotate_right(&mut self, angle: f32) {
        self.a += angle;
    }
}
