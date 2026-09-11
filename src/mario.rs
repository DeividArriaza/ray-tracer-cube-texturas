use crate::color::Color;
use crate::texture::Texture;

// Paleta aproximada de Super Mario Bros. (NES). Cada sprite usa un subconjunto.
const OUTLINE: Color = Color::new(0, 0, 0);

// Bloque de interrogación: naranja con realce arriba y sombra abajo.
const BLOCK_LIGHT: Color = Color::new(252, 188, 60);
const BLOCK_BASE: Color = Color::new(231, 156, 33);
const BLOCK_SHADOW: Color = Color::new(166, 92, 0);
const BLOCK_GLYPH: Color = Color::new(80, 40, 0);
const RIVET: Color = Color::new(252, 252, 252);

// Ladrillo: rojo anaranjado con junta de mortero oscura.
const BRICK_LIGHT: Color = Color::new(228, 108, 44);
const BRICK_BASE: Color = Color::new(200, 76, 12);
const BRICK_SHADOW: Color = Color::new(148, 52, 8);
const MORTAR: Color = Color::new(32, 16, 8);

/// Bloque `?` de 16 x 16: marco negro, cuatro remaches y el signo de
/// interrogación centrado. Se aplica a las seis caras del cubo.
#[rustfmt::skip]
const QUESTION_BLOCK: [&str; 16] = [
    "oooooooooooooooo",
    "oLLLLLLLLLLLLLLo",
    "oLwKKKKKKKKKKwko",
    "oLKKKKddddKKKKko",
    "oLKKKddddddKKKko",
    "oLKKKddKKddKKKko",
    "oLKKKKKKKddKKKko",
    "oLKKKKKKdddKKKko",
    "oLKKKKKdddKKKKko",
    "oLKKKKKddKKKKKko",
    "oLKKKKKddKKKKKko",
    "oLKKKKKKKKKKKKko",
    "oLKKKKKddKKKKKko",
    "oLwKKKKKKKKKKwko",
    "okkkkkkkkkkkkkko",
    "oooooooooooooooo",
];

/// Ladrillo de 16 x 16: cuatro hiladas de tres píxeles de alto separadas por
/// juntas de mortero, con la junta vertical desplazada en hiladas alternas para
/// que el patrón no se alinee al repetirse.
#[rustfmt::skip]
const BRICK: [&str; 16] = [
    "oooooooooooooooo",
    "LLLLLLLoLLLLLLLL",
    "BBBBBBBoBBBBBBBB",
    "bbbbbbbobbbbbbbb",
    "oooooooooooooooo",
    "LLLLLLLLLLLLLLLo",
    "BBBBBBBBBBBBBBBo",
    "bbbbbbbbbbbbbbbo",
    "oooooooooooooooo",
    "LLLLLLLoLLLLLLLL",
    "BBBBBBBoBBBBBBBB",
    "bbbbbbbobbbbbbbb",
    "oooooooooooooooo",
    "LLLLLLLLLLLLLLLo",
    "BBBBBBBBBBBBBBBo",
    "bbbbbbbbbbbbbbbo",
];

pub fn question_block() -> Texture {
    Texture::from_pixel_art(
        &QUESTION_BLOCK,
        &[
            ('o', OUTLINE),
            ('L', BLOCK_LIGHT),
            ('K', BLOCK_BASE),
            ('k', BLOCK_SHADOW),
            ('d', BLOCK_GLYPH),
            ('w', RIVET),
        ],
    )
}

pub fn brick() -> Texture {
    Texture::from_pixel_art(
        &BRICK,
        &[
            ('o', MORTAR),
            ('L', BRICK_LIGHT),
            ('B', BRICK_BASE),
            ('b', BRICK_SHADOW),
        ],
    )
}
