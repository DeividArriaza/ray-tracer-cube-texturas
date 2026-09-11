use crate::ray_intersect::{Intersect, Material, RayIntersect};
use nalgebra_glm::{Vec2, Vec3};

const EPSILON: f32 = 1e-4;

pub struct Cube {
    pub min: Vec3,
    pub max: Vec3,
    pub material: Material,
}

impl Cube {
    pub fn new(center: Vec3, size: f32, material: Material) -> Self {
        let half = Vec3::new(size, size, size) * 0.5;

        Cube {
            min: center - half,
            max: center + half,
            material,
        }
    }

    /// Coordenadas de textura del punto sobre la cara golpeada. El cubo se
    /// trata como seis caras independientes y cada una recibe el sprite
    /// completo; el eje que se invierte en cada caso es el que deja el dibujo
    /// derecho y sin espejar visto desde fuera del cubo.
    fn face_uv(&self, point: &Vec3, axis: usize, sign: f32) -> Vec2 {
        // Posición dentro de la caja, normalizada a [0, 1] en los tres ejes.
        let local = (point - self.min).component_div(&(self.max - self.min));

        let (u, v) = match (axis, sign > 0.0) {
            (0, true) => (1.0 - local.z, 1.0 - local.y),  // cara +X (derecha)
            (0, false) => (local.z, 1.0 - local.y),       // cara -X (izquierda)
            (1, true) => (local.x, local.z),              // cara +Y (arriba)
            (1, false) => (local.x, 1.0 - local.z),       // cara -Y (abajo)
            (2, true) => (local.x, 1.0 - local.y),        // cara +Z (frente)
            _ => (1.0 - local.x, 1.0 - local.y),          // cara -Z (atrás)
        };

        Vec2::new(u, v)
    }
}

fn axis_normal(axis: usize, sign: f32) -> Vec3 {
    let mut normal = Vec3::zeros();
    normal[axis] = sign;
    normal
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersect> {
        // Slab method: the ray hits the box only where the three per-axis
        // intervals [t_near, t_far] overlap.
        let mut t_min = f32::NEG_INFINITY;
        let mut t_max = f32::INFINITY;

        let mut axis_min = 0;
        let mut axis_max = 0;

        for axis in 0..3 {
            let origin = ray_origin[axis];
            let direction = ray_direction[axis];

            if direction.abs() < EPSILON {
                if origin < self.min[axis] || origin > self.max[axis] {
                    return None;
                }

                continue;
            }

            let inverse = 1.0 / direction;

            let mut t_near = (self.min[axis] - origin) * inverse;
            let mut t_far = (self.max[axis] - origin) * inverse;

            if t_near > t_far {
                std::mem::swap(&mut t_near, &mut t_far);
            }

            if t_near > t_min {
                t_min = t_near;
                axis_min = axis;
            }

            if t_far < t_max {
                t_max = t_far;
                axis_max = axis;
            }

            if t_min > t_max {
                return None;
            }
        }

        // t_min is the entry face; when the origin is inside the cube the
        // first visible face is the exit one.
        let (distance, axis, facing) = if t_min > EPSILON {
            (t_min, axis_min, -1.0)
        } else if t_max > EPSILON {
            (t_max, axis_max, 1.0)
        } else {
            return None;
        };

        let sign = if ray_direction[axis] < 0.0 {
            -facing
        } else {
            facing
        };

        let point = ray_origin + ray_direction * distance;

        Some(Intersect {
            uv: self.face_uv(&point, axis, sign),
            point,
            normal: axis_normal(axis, sign),
            distance,
            material: self.material.clone(),
        })
    }
}
