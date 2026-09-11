# ray-tracer-cube-texturas

Raytracer en Rust con **texturas**: un bloque `?` de Super Mario Bros. apoyado
sobre un piso de ladrillo, iluminación **difusa** (ley de Lambert) con una luz
puntual elevada, **sombra proyectada** del cubo sobre el piso y una **cámara
orbital** controlada con el teclado.

Las texturas son *pixel art* procedural definido en el propio código: no hay
archivos de imagen ni dependencias nuevas, el repositorio corre con solo
`cargo run`. Cada sprite es una rejilla de 16 x 16 caracteres que se traduce a
color con una paleta.

Continúa el proyecto `ray-tracer-cube`, al que añade el sistema de texturas.

## Ejecutar

```bash
cargo run --release
```

## Controles

| Tecla | Acción |
|-------|--------|
| ← / → | Órbita en yaw (horizontal) alrededor del cubo |
| ↑ / ↓ | Órbita en pitch (vertical), limitada a ±(π/2 − 0.1) |
| Esc   | Salir |

El render se recalcula solo cuando la cámara se mueve.

## Estructura

- `src/texture.rs` — `Texture`: mapa de bits construido desde arte ASCII
  (`from_pixel_art`) y muestreado por vecino más cercano con repetición
  (`sample`).
- `src/mario.rs` — los dos sprites de 16 x 16 (bloque `?` y ladrillo) con la
  paleta de Super Mario Bros.
- `src/ray_intersect.rs` — `Surface` (color plano o textura), `Material`
  (superficie + albedo, con `diffuse(uv)`), `Intersect` (ahora con `uv`) y el
  trait `RayIntersect`.
- `src/cube.rs` — intersección rayo/cubo por el método de *slabs*, con normal y
  coordenadas de textura por cara (`face_uv`).
- `src/plane.rs` — superficie finita con alicatado controlado por `tile_size`.
- `src/main.rs` — loop de ventana, `render`, `cast_ray`, `cast_shadow` y `shade`.
- `src/camera.rs`, `src/light.rs`, `src/color.rs`, `src/framebuffer.rs` — cámara
  orbital, luz puntual, color y buffer de píxeles.

## Cómo funcionan las texturas

### Definición del sprite

Cada textura se escribe como arte ASCII y se acompaña de una paleta que asigna
un color a cada símbolo:

```rust
const QUESTION_BLOCK: [&str; 16] = [
    "oooooooooooooooo",
    "oLLLLLLLLLLLLLLo",
    "oLwKKKKKKKKKKwko",
    // ...
];
```

`o` es el contorno negro, `L` el realce superior, `K` el naranja base, `k` la
sombra inferior, `d` el trazo del signo `?` y `w` los cuatro remaches blancos.

### Muestreo

`Texture::sample(u, v)` toma coordenadas normalizadas con `v = 0` en la fila
superior y devuelve el color del píxel más cercano, sin interpolar: eso es lo
que conserva el borde duro del pixel art en vez de difuminarlo. Fuera de
`[0, 1)` las coordenadas se repiten (`u - u.floor()`), y esa repetición es la
que permite alicatar el piso entero con un único sprite.

### Coordenadas de textura del cubo

El cubo se trata como seis caras independientes y cada una recibe el sprite
completo. Como el método de *slabs* ya devuelve el eje y el signo de la cara
golpeada, `face_uv` normaliza el punto de impacto dentro de la caja y elige qué
dos componentes son `u` y `v`, invirtiendo la que haga falta para que el `?`
quede derecho y sin espejar visto desde fuera:

| Cara | u | v |
|------|---|---|
| +X (derecha) | `1 - z` | `1 - y` |
| −X (izquierda) | `z` | `1 - y` |
| +Y (arriba) | `x` | `z` |
| −Y (abajo) | `x` | `1 - z` |
| +Z (frente) | `x` | `1 - y` |
| −Z (atrás) | `1 - x` | `1 - y` |

### Coordenadas de textura del plano

El plano proyecta el desplazamiento desde su centro sobre sus dos ejes y divide
entre `tile_size`, el lado en unidades de mundo que ocupa una repetición. Con
`tile_size = 1.0` el piso de 14 x 14 muestra 14 x 14 ladrillos.

### Materiales

`Material` deja de guardar un color fijo y guarda una `Surface`, que es un color
plano o una textura. La textura se comparte con `Arc` porque el material se
clona en cada intersección; copiar los píxeles por rayo sería inviable.

```rust
let question_block = Material::textured(Arc::new(mario::question_block()), 0.9);
let brick = Material::textured(Arc::new(mario::brick()), 0.7);
```

`shade` pide el color con `intersect.material.diffuse(intersect.uv)` en lugar de
leer un campo, y el resto del modelo de iluminación queda igual.

## Escena

- Cubo de lado 2 centrado en el origen, con el sprite del bloque `?` en sus seis
  caras; su cara inferior queda en `y = -1`.
- Superficie finita de 14 x 14 en `y = -1`, normal `(0, 1, 0)`, con el ladrillo
  repetido cada unidad de mundo.
- Luz puntual en `(-4.0, 6.0, 4.0)` apuntando al cubo, intensidad 1.3.
- Cámara inicial en `(4.5, 3.5, 6.5)` mirando al centro del cubo.
- Fondo `0x5C94FC`, el azul de cielo de Super Mario Bros.

## Sombras

Por cada punto visible se lanza un rayo hacia la luz (`cast_shadow` en
`src/main.rs`). Si algún objeto lo bloquea antes de llegar a la luz, el punto
queda en sombra. Dos constantes controlan el resultado:

- `SHADOW_BIAS = 1e-3` — desplaza el origen del rayo sobre la normal para que la
  superficie no se sombree a sí misma (evita el *shadow acne*).
- `SHADOW_INTENSITY = 0.7` — cuánta luz se pierde en la sombra; `1.0` la deja
  completamente negra.

## Nota sobre el sombreado

El sombreado sigue siendo únicamente difuso: `intensidad = max(0, n · l)`, sin
componente especular ni ambiental. Por eso las caras que no ven la luz quedan en
negro al orbitar y su textura no se aprecia — es el comportamiento esperado de
Lambert con una sola luz puntual, heredado del proyecto anterior. Un término
ambiental pequeño bastaría para que la textura se leyera en las caras oscuras.
