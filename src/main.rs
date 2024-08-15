mod framebuffer;
mod maze;
mod player;
mod caster;
mod texture;


use minifb::{Window, WindowOptions, Key};
use nalgebra_glm::{Vec2};
use std::f32::consts::PI;
use std::time::{Instant, Duration};
use crate::framebuffer::{Framebuffer, Color};
use crate::maze::load_maze;
use crate::player::{Player};
use crate::caster::{cast_ray, render_3d};
use crate::texture::Texture;


fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize, cell: char) {
    if cell == ' ' {
        return;
    }

    framebuffer.set_current_color(Color(0xFF0000));

    for x in xo..xo + block_size {
        for y in yo..yo + block_size {
            framebuffer.point(x, y);
        }
    }
}

fn render(framebuffer: &mut Framebuffer, player: &Player, maze: &Vec<Vec<char>>) {
    let block_size = 100;

    // draws maze
    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            draw_cell(framebuffer, col * block_size, row * block_size, block_size, maze[row][col])
        }
    }

    framebuffer.set_current_color(Color(0xFF0000));
    framebuffer.point(player.pos.x as usize, player.pos.y as usize);

    // cast ray
    let num_rays = 5;
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);

        cast_ray(framebuffer, &maze, &player, a, block_size);
    }
}

fn main() {
    let window_width = 1300;
    let window_height = 900;
    let framebuffer_width = 1300;
    let framebuffer_height = 900;
    let frame_delay = Duration::from_millis(16);
    let mut last_time = Instant::now();
    let mut frames = 0;
    let mut fps = 0;

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Maze Runner",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    framebuffer.set_background_color(Color(0x333355));

    let mut player = Player {
        pos: Vec2::new(150.0, 150.0),
        a: PI / 3.0,
        fov: PI / 3.0,
        speed: 2.0, 
    };

    let maze = load_maze("./maze.txt");
    let texture = Texture::from_file("back4.jpg").expect("Failed to load texture");


    let mut is_3d_mode = false; // Variable para controlar el modo

    while window.is_open() && !window.is_key_down(Key::Escape) {

        if window.is_key_down(Key::M) {
            is_3d_mode = !is_3d_mode; // Alterna entre 2D y 3D
        }

        // Movimiento del jugador con colisiones
        if window.is_key_down(Key::W) {
            let next_pos = Vec2::new(player.pos.x + player.a.cos() * player.speed, player.pos.y + player.a.sin() * player.speed);
            if !is_colliding(&next_pos, &maze, 100) {
                player.pos = next_pos;
            }
        }
        if window.is_key_down(Key::S) {
            let next_pos = Vec2::new(player.pos.x - player.a.cos() * player.speed, player.pos.y - player.a.sin() * player.speed);
            if !is_colliding(&next_pos, &maze, 100) {
                player.pos = next_pos;
            }
        }
        if window.is_key_down(Key::A) {
            player.a -= 0.05;
        }
        if window.is_key_down(Key::D) {
            player.a += 0.05;
        }

        let current_time = Instant::now();
        let delta_time = current_time.duration_since(last_time);
        frames += 1;
        if delta_time >= Duration::from_secs(1) {
            fps = frames;
            frames = 0;
            last_time = current_time;
        }

        framebuffer.clear();

        if is_3d_mode {
            render_3d(&mut framebuffer, &player, &maze, &texture);
        } else {
            render(&mut framebuffer, &player, &maze);
        }

        let fps_text = format!("FPS: {}", fps);
        framebuffer.draw_text(framebuffer.width - 100, 10, &fps_text, Color(0xFFFFFF));

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}

// Nueva función para verificar colisiones
fn is_colliding(pos: &Vec2, maze: &Vec<Vec<char>>, block_size: usize) -> bool {
    let x = pos.x as usize;
    let y = pos.y as usize;

    if x >= maze[0].len() * block_size || y >= maze.len() * block_size {
        return true;
    }

    let i = y / block_size;
    let j = x / block_size;

    maze[i][j] != ' '
}
