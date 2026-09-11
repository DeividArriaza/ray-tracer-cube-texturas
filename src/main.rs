mod camera;
mod color;
mod cube;
mod framebuffer;
mod light;
mod mario;
mod plane;
mod ray_intersect;
mod texture;

use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{dot, normalize, Vec3};
use std::f32::consts::PI;
use std::sync::Arc;
use std::time::Duration;

use crate::camera::Camera;
use crate::color::Color;
use crate::cube::Cube;
use crate::framebuffer::Framebuffer;
use crate::light::Light;
use crate::plane::Plane;
use crate::ray_intersect::{Intersect, Material, RayIntersect};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const BACKGROUND_COLOR: u32 = 0x5C94FC;

const FOV: f32 = PI / 3.0;

const ROTATION_SPEED: f32 = PI / 60.0;

// Desplazamiento del origen del rayo de sombra sobre la normal, para que la
// superficie no se sombree a sí misma.
const SHADOW_BIAS: f32 = 1e-3;
// 1.0 = sombra totalmente negra; menos deja la sombra más suave.
const SHADOW_INTENSITY: f32 = 0.7;

/// Lanza un rayo desde el punto hacia la luz: si algo lo bloquea antes de
/// llegar, el punto está en sombra.
pub fn cast_shadow(
    intersect: &Intersect,
    light: &Light,
    objects: &[Box<dyn RayIntersect>],
) -> f32 {
    let light_vector = light.position - intersect.point;
    let light_distance = light_vector.magnitude();
    let light_direction = light_vector / light_distance;

    let shadow_origin = intersect.point + intersect.normal * SHADOW_BIAS;

    for object in objects {
        if let Some(blocker) = object.ray_intersect(&shadow_origin, &light_direction) {
            if blocker.distance < light_distance {
                return SHADOW_INTENSITY;
            }
        }
    }

    0.0
}

/// Difuso puro (ley de Lambert) con sombra proyectada: sin componente especular.
pub fn shade(intersect: &Intersect, light: &Light, objects: &[Box<dyn RayIntersect>]) -> Color {
    let light_direction = (light.position - intersect.point).normalize();

    let diffuse_intensity = dot(&intersect.normal, &light_direction).max(0.0);

    if diffuse_intensity <= 0.0 {
        return Color::new(0, 0, 0);
    }

    let light_reaching = 1.0 - cast_shadow(intersect, light, objects);

    intersect.material.diffuse(intersect.uv)
        * light.color
        * (diffuse_intensity * intersect.material.albedo * light.intensity * light_reaching)
}

pub fn cast_ray(
    ray_origin: &Vec3,
    ray_direction: &Vec3,
    objects: &[Box<dyn RayIntersect>],
    light: &Light,
) -> Color {
    let mut closest: Option<Intersect> = None;

    for object in objects {
        if let Some(hit) = object.ray_intersect(ray_origin, ray_direction) {
            let is_closer = closest
                .as_ref()
                .is_none_or(|current: &Intersect| hit.distance < current.distance);

            if is_closer {
                closest = Some(hit);
            }
        }
    }

    match closest {
        Some(intersect) => shade(&intersect, light, objects),
        None => Color::from_hex(BACKGROUND_COLOR),
    }
}

pub fn render(
    framebuffer: &mut Framebuffer,
    objects: &[Box<dyn RayIntersect>],
    camera: &Camera,
    light: &Light,
) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;

    let perspective_scale = (FOV / 2.0).tan();

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            let screen_x = screen_x * aspect_ratio * perspective_scale;
            let screen_y = screen_y * perspective_scale;

            let ray_direction = normalize(&Vec3::new(screen_x, screen_y, -1.0));
            let ray_direction = camera.basis_change(&ray_direction);

            framebuffer
                .set_current_color(cast_ray(&camera.eye, &ray_direction, objects, light).to_hex());
            framebuffer.point(x, y);
        }
    }
}

fn main() {
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    framebuffer.set_background_color(BACKGROUND_COLOR);
    framebuffer.clear();

    let mut window = Window::new(
        "Ray Tracer - Bloque ? de Mario con texturas",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    // Las texturas se comparten con Arc porque cada material se clona en cada
    // intersección; copiar los píxeles por rayo sería inviable.
    let question_block = Material::textured(Arc::new(mario::question_block()), 0.9);
    let brick = Material::textured(Arc::new(mario::brick()), 0.7);

    // El cubo (lado 2, centrado en el origen) apoya su cara inferior en y = -1,
    // que es el nivel de la superficie finita de 14 x 14. Con tile_size = 1.0 el
    // piso muestra 14 x 14 ladrillos, uno por unidad de mundo.
    let objects: Vec<Box<dyn RayIntersect>> = vec![
        Box::new(Cube::new(Vec3::new(0.0, 0.0, 0.0), 2.0, question_block)),
        Box::new(Plane::new(
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            7.0,
            7.0,
            1.0,
            brick,
        )),
    ];

    // Luz puntual elevada apuntando al cubo: proyecta su sombra sobre la
    // superficie, y al orbitar se aprecia la sombra desde distintos ángulos.
    let light = Light::new(Vec3::new(-4.0, 6.0, 4.0), Color::new(255, 255, 255), 1.3);

    let mut camera = Camera::new(
        Vec3::new(4.5, 3.5, 6.5),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let mut camera_moved = true;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let orbit = [
            (Key::Left, ROTATION_SPEED, 0.0),
            (Key::Right, -ROTATION_SPEED, 0.0),
            (Key::Up, 0.0, -ROTATION_SPEED),
            (Key::Down, 0.0, ROTATION_SPEED),
        ];

        for (key, delta_yaw, delta_pitch) in orbit {
            if window.is_key_down(key) {
                camera.orbit(delta_yaw, delta_pitch);
                camera_moved = true;
            }
        }

        if camera_moved {
            render(&mut framebuffer, &objects, &camera, &light);
            camera_moved = false;
        }

        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
