use std::collections::HashMap;
use cgmath::{InnerSpace, Point3, Vector2, Vector3};

#[derive(Debug)]
pub struct TexParams {
    pub off: Vector2<f32>,
    pub rot: f32,
    pub scale: Vector2<f32>,
}

#[derive(Debug)]
pub struct BrushPlane {
    pub p: Point3<f32>,
    pub q: Point3<f32>,
    pub r: Point3<f32>,
    pub texname: String, // TODO intern lmfao
    pub texparams: TexParams,
}

impl BrushPlane {
    // n = normalize((r - p) × (q - p))
    pub fn normal(&self) -> Vector3<f32> {
        (self.r-self.p).cross(self.q-self.p).normalize()
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
