use crate::framebuffer::{Framebuffer, Color};
use crate::player::Player;
use crate::texture::Texture;


pub fn cast_ray(framebuffer: &mut Framebuffer, maze: &Vec<Vec<char>>, player: &Player, a: f32, block_size: usize) {
    let mut d = 0.0;

    framebuffer.set_current_color(Color(0x000000));

    loop {
        let cos = d * a.cos();
        let sin = d * a.sin();

        let x = (player.pos.x + cos) as usize;
        let y = (player.pos.y + sin) as usize;

        if x >= maze[0].len() * block_size || y >= maze.len() * block_size {
            break;
        }

        let i = y / block_size;
        let j = x / block_size;

        if maze[i][j] != ' ' {
            break;
        }

        framebuffer.set_current_color(Color(0xFFDDDD)); 
        framebuffer.point(x, y); 

        d += 0.1; 
    }
}
pub fn render_3d(framebuffer: &mut Framebuffer, player: &Player, maze: &Vec<Vec<char>>, texture: &Texture) {
    let block_size = 100;
    let num_rays = framebuffer.width;
    
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        
        let (distance, hit_wall_x, hit_wall_y) = cast_ray_3d(&maze, &player, a, block_size);
        let distance = distance * (player.a - a).cos(); 
        
        let distance = if distance == 0.0 { 0.0001 } else { distance };
        
        let wall_height = (framebuffer.height as f32 / distance) as usize;
        let wall_start = (framebuffer.height / 2).saturating_sub(wall_height / 2);
        let wall_end = (framebuffer.height / 2).saturating_add(wall_height / 2);

        let texture_x = if (hit_wall_x % block_size) < 5 || (hit_wall_x % block_size) > (block_size - 5) {
            hit_wall_y as f32 / block_size as f32
        } else {
            hit_wall_x as f32 / block_size as f32
        };

        for y in wall_start..wall_end {
            let texture_y = (y as f32 - wall_start as f32) / wall_height as f32;
            let color = texture.get_color_interpolated(texture_x, texture_y);
            framebuffer.set_current_color(Color(color));
            framebuffer.point(i, y);
        }
    }
}




fn cast_ray_3d(maze: &Vec<Vec<char>>, player: &Player, a: f32, block_size: usize) -> (f32, usize, usize) {
    let mut d = 0.0;

    loop {
        let cos = d * a.cos();
        let sin = d * a.sin();

        let x = (player.pos.x + cos) as usize;
        let y = (player.pos.y + sin) as usize;

        let maze_x = x / block_size;
        let maze_y = y / block_size;

        if maze_x >= maze[0].len() || maze_y >= maze.len() {
            break (d, x, y);
        }

        let cell = maze[maze_y][maze_x];
        if cell == '|' || cell == '-' || cell == '+' {
            break (d, x % block_size, y % block_size);
        }

        d += 1.0; 
    }
}

