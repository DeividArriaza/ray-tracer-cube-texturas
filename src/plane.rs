use crate::ray_intersect::{Intersect, Material, RayIntersect};
use nalgebra_glm::{dot, Vec2, Vec3};

const EPSILON: f32 = 1e-4;

/// Superficie finita (rectángulo) definida por su centro, su normal y dos
/// semiejes sobre el plano.
pub struct Plane {
    pub center: Vec3,
    pub normal: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub half_u: f32,
    pub half_v: f32,
    /// Lado, en unidades de mundo, que ocupa una repetición de la textura.
    pub tile_size: f32,
    pub material: Material,
}

impl Plane {
    pub fn new(
        center: Vec3,
        normal: Vec3,
        u: Vec3,
        half_u: f32,
        half_v: f32,
        tile_size: f32,
        material: Material,
    ) -> Self {
        let normal = normal.normalize();

        // Se ortogonaliza u contra la normal para que u, v y normal formen una
        // base ortonormal aunque u venga inclinado.
        let u = (u - normal * dot(&u, &normal)).normalize();
        let v = normal.cross(&u).normalize();

        Plane {
            center,
            normal,
            u,
            v,
            half_u,
            half_v,
            tile_size,
            material,
        }
    }
}

impl RayIntersect for Plane {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersect> {
        let denominator = dot(ray_direction, &self.normal);

        // Rayo paralelo al plano: no hay intersección.
        if denominator.abs() < EPSILON {
            return None;
        }

        let distance = dot(&(self.center - ray_origin), &self.normal) / denominator;

        if distance <= EPSILON {
            return None;
        }

        let point = ray_origin + ray_direction * distance;
        let offset = point - self.center;

        // Fuera del rectángulo: el plano es finito.
        if dot(&offset, &self.u).abs() > self.half_u || dot(&offset, &self.v).abs() > self.half_v {
            return None;
        }

        // La normal se orienta hacia el rayo para que la cara visible reciba
        // luz difusa correctamente desde ambos lados.
        let normal = if denominator < 0.0 {
            self.normal
        } else {
            -self.normal
        };

        // El alicatado sale de proyectar el desplazamiento sobre los ejes del
        // plano: el muestreo repite la textura fuera de [0, 1), así que basta
        // con dividir entre el lado del tile.
        let uv = Vec2::new(
            dot(&offset, &self.u) / self.tile_size,
            dot(&offset, &self.v) / self.tile_size,
        );

        Some(Intersect {
            point,
            normal,
            distance,
            uv,
            material: self.material.clone(),
        })
    }
}
