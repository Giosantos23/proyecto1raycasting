use nalgebra_glm::{Vec2};

pub struct Player {
    pub pos: Vec2,
    pub a: f32,
    pub fov: f32,
    pub speed: f32, // Velocidad del jugador
}