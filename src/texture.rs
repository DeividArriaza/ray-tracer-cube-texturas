use crate::color::Color;

/// Mapa de bits muestreado por vecino más cercano: los píxeles se leen tal
/// cual, sin interpolar, para conservar el borde duro del pixel art.
#[derive(Debug)]
pub struct Texture {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Color>,
}

impl Texture {
    /// Construye la textura a partir de arte ASCII: cada carácter de `rows` se
    /// traduce con `palette`. Todas las filas deben medir lo mismo y todos los
    /// símbolos deben existir en la paleta.
    pub fn from_pixel_art(rows: &[&str], palette: &[(char, Color)]) -> Self {
        let height = rows.len();
        let width = rows[0].chars().count();

        let mut pixels = Vec::with_capacity(width * height);

        for (y, row) in rows.iter().enumerate() {
            assert_eq!(
                row.chars().count(),
                width,
                "la fila {y} no mide {width} caracteres"
            );

            for symbol in row.chars() {
                let color = palette
                    .iter()
                    .find(|(key, _)| *key == symbol)
                    .map(|(_, color)| *color)
                    .unwrap_or_else(|| panic!("símbolo '{symbol}' fuera de la paleta"));

                pixels.push(color);
            }
        }

        Texture {
            width,
            height,
            pixels,
        }
    }

    /// Muestrea con coordenadas normalizadas, donde `v = 0` es la fila
    /// superior. Fuera de `[0, 1)` las coordenadas se repiten, que es lo que
    /// permite alicatar el piso entero con un solo sprite.
    pub fn sample(&self, u: f32, v: f32) -> Color {
        let u = u - u.floor();
        let v = v - v.floor();

        let x = ((u * self.width as f32) as usize).min(self.width - 1);
        let y = ((v * self.height as f32) as usize).min(self.height - 1);

        self.pixels[y * self.width + x]
    }
}
