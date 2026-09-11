use crate::color::Color;
use crate::texture::Texture;
use nalgebra_glm::{Vec2, Vec3};
use std::sync::Arc;

/// De dónde saca un material su color difuso en el punto de impacto.
#[derive(Debug, Clone)]
pub enum Surface {
    /// Color plano: ignora las coordenadas de textura.
    Solid(Color),
    /// Mapa de bits muestreado con las coordenadas (u, v) de la intersección.
    Textured(Arc<Texture>),
}

#[derive(Debug, Clone)]
pub struct Material {
    pub surface: Surface,
    pub albedo: f32,
}

impl Material {
    pub fn solid(diffuse: Color, albedo: f32) -> Self {
        Material {
            surface: Surface::Solid(diffuse),
            albedo,
        }
    }

    pub fn textured(texture: Arc<Texture>, albedo: f32) -> Self {
        Material {
            surface: Surface::Textured(texture),
            albedo,
        }
    }

    /// Color difuso en unas coordenadas de textura concretas.
    pub fn diffuse(&self, uv: Vec2) -> Color {
        match &self.surface {
            Surface::Solid(color) => *color,
            Surface::Textured(texture) => texture.sample(uv.x, uv.y),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Intersect {
    pub point: Vec3,
    pub normal: Vec3,
    pub distance: f32,
    /// Coordenadas de textura del punto, con `v = 0` en la fila superior.
    pub uv: Vec2,
    pub material: Material,
}

pub trait RayIntersect {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersect>;
}
