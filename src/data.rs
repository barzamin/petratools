use std::collections::HashMap;
use ultraviolet::{Vec2, Vec3};

#[derive(Debug)]
pub struct TexParams {
    pub off: Vec2,
    pub rot: f32,
    pub scale: Vec2,
}

#[derive(Debug)]
pub struct BrushPlane {
    pub p: Vec3,
    pub q: Vec3,
    pub r: Vec3,
    pub texname: String, // TODO intern lmfao
    pub texparams: TexParams,
}

impl BrushPlane {
    // n = normalize((r - p) × (q - p))
    pub fn normal(&self) -> Vec3 {
        (self.r-self.p).cross(self.q-self.p).normalized()
    }
}

#[derive(Debug)]
pub struct Brush {
    pub planes: Vec<BrushPlane>,
}

#[derive(Debug)]
pub struct Entity {
    pub keys: HashMap<String, String>,
    pub brushes: Vec<Brush>,
}

#[derive(Debug)]
pub struct Map {
    pub entities: Vec<Entity>,
}
