# colorgrad-rs

[![crates.io](https://img.shields.io/crates/v/colorgrad.svg)](https://crates.io/crates/colorgrad)
[![Documentation](https://docs.rs/colorgrad/badge.svg)](https://docs.rs/colorgrad)
[![Build Status](https://github.com/mazznoer/colorgrad-rs/workflows/Rust/badge.svg)](https://github.com/mazznoer/colorgrad-rs/actions)
[![Build Status](https://travis-ci.org/mazznoer/colorgrad-rs.svg?branch=master)](https://travis-ci.org/mazznoer/colorgrad-rs)
[![codecov](https://codecov.io/gh/mazznoer/colorgrad-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/mazznoer/colorgrad-rs)

Rust color scales library for charts, maps, data-visualization and creative coding.

## Index

+ [Custom Gradient](#custom-gradient)
+ [Preset Gradients](#preset-gradients)
+ [Hard-Edged Gradient](#hard-edged-gradient)
+ [Examples](#examples)

## Usage

Add `colorgrad` to your `Cargo.toml`
```
[dependencies]
colorgrad = "0.1.0"
```

## Custom Gradient

### Basic

```rust
let g = colorgrad::CustomGradient::new().build().unwrap();
```
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/custom-default.png)

### Custom Colors

```rust
use colorgrad::Color;

let g = colorgrad::CustomGradient::new()
    .colors(&[
        Color::from_rgb_u8(0, 206, 209),
        Color::from_rgb_u8(255, 105, 180),
        Color::from_rgb(0.274, 0.5, 0.7),
        Color::from_hsv(50., 1., 1.),
        Color::from_hsv(348., 0.9, 0.8),
    ])
    .build()
    .unwrap();
```
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/custom-colors.png)

### Using Web Color Format

`.html_colors()` method accepts named colors, hexadecimal (`#rgb`, `#rgba`, `#rrggbb`, `#rrggbbaa`), `rgb()`, `rgba()`, `hsl()`, `hsla()`, `hwb()`, and `hsv()`.

```rust
let g = colorgrad::CustomGradient::new()
    .html_colors(&["#c41189", "#00BFFF", "#FFD700"])
    .build()
    .unwrap();
```
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/custom-hex-colors.png)

```rust
let g = colorgrad::CustomGradient::new()
    .html_colors(&["gold", "hotpink", "darkturquoise"])
    .build()
    .unwrap();
```
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/custom-named-colors.png)

```rust
let g = colorgrad::CustomGradient::new()
    .html_colors(&["rgb(125,110,221)", "rgb(90%,45%,97%)", "hsl(229,79%,85%)"])
    .build()
    .unwrap();
```
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/custom-css-colors.png)

### Domain & Color Position

```rust
let g = colorgrad::CustomGradient::new()
    .html_colors(&["deeppink", "gold", "seagreen"])
    .build()
    .unwrap();

assert_eq!(g.domain(), (0., 1.));
```
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/domain-default.png)

```rust
let g = colorgrad::CustomGradient::new()
    .html_colors(&["deeppink", "gold", "seagreen"])
    .domain(&[0., 100.])
    .build()
    .unwrap();

assert_eq!(g.domain(), (0., 100.));
```
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/domain-100.png)

```rust
let g = colorgrad::CustomGradient::new()
    .html_colors(&["deeppink", "gold", "seagreen"])
    .domain(&[-1., 1.])
    .build()
    .unwrap();

assert_eq!(g.domain(), (-1., 1.));
```
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/domain-neg1-1.png)

```rust
let g = colorgrad::CustomGradient::new()
    .html_colors(&["deeppink", "gold", "seagreen"])
    .domain(&[0., 0.7, 1.])
    .build()
    .unwrap();

assert_eq!(g.domain(), (0., 1.));
```
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/color-position-1.png)

```rust
let g = colorgrad::CustomGradient::new()
    .html_colors(&["deeppink", "gold", "seagreen"])
    .domain(&[15., 30., 80.])
    .build()
    .unwrap();

assert_eq!(g.domain(), (15., 80.));
```
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/color-position-2.png)

### Blending Mode

```rust
let g = colorgrad::CustomGradient::new()
    .html_colors(&["#ff0", "#008ae5"])
    .mode(colorgrad::BlendMode::Rgb)
    .build()
    .unwrap();
```
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/blend-mode-rgb.png)

```rust
let g = colorgrad::CustomGradient::new()
    .html_colors(&["#ff0", "#008ae5"])
    .mode(colorgrad::BlendMode::Lrgb)
    .build()
    .unwrap();
```
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/blend-mode-lrgb.png)

```rust
let g = colorgrad::CustomGradient::new()
    .html_colors(&["#ff0", "#008ae5"])
    .mode(colorgrad::BlendMode::Hsv)
    .build()
    .unwrap();
```
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/blend-mode-hsv.png)

## Preset Gradients

All preset gradients are in the domain 0..1.

### Diverging

`colorgrad::brbg()`
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/brbg.png)

`colorgrad::prgn()`
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/prgn.png)

`colorgrad::piyg()`
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/piyg.png)

`colorgrad::puor()`
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/puor.png)

`colorgrad::rdbu()`
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/rdbu.png)

`colorgrad::rdgy()`
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/rdgy.png)

`colorgrad::rdylbu()`
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/rdylbu.png)

`colorgrad::rdylgn()`
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/rdylgn.png)

`colorgrad::spectral()`
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/spectral.png)

### Sequential (Single Hue)

`colorgrad::blues()`
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/blues.png)

`colorgrad::greens()`
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/greens.png)

`colorgrad::greys()`
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/greys.png)

`colorgrad::oranges()`
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/oranges.png)

`colorgrad::purples()`
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/purples.png)

`colorgrad::reds()`
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/reds.png)

### Sequential (Multi-Hue)

`colorgrad::turbo()`
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/turbo.png)

`colorgrad::viridis()`
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/viridis.png)

`colorgrad::inferno()`
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/inferno.png)

`colorgrad::magma()`
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/magma.png)

`colorgrad::plasma()`
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/plasma.png)

`colorgrad::cividis()`
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/cividis.png)

`colorgrad::warm()`
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/warm.png)

`colorgrad::cool()`
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/cool.png)

`colorgrad::cubehelix_default()`
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/cubehelix_default.png)

`colorgrad::bugn()`
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/bugn.png)

`colorgrad::bupu()`
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/bupu.png)

`colorgrad::gnbu()`
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/gnbu.png)

`colorgrad::orrd()`
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/orrd.png)

`colorgrad::pubugn()`
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/pubugn.png)

`colorgrad::pubu()`
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/pubu.png)

`colorgrad::purd()`
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/purd.png)

`colorgrad::rdpu()`
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/rdpu.png)

`colorgrad::ylgnbu()`
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/ylgnbu.png)

`colorgrad::ylgn()`
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/ylgn.png)

`colorgrad::ylorbr()`
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/ylorbr.png)

`colorgrad::ylorrd()`
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/ylorrd.png)

### Cyclical

`colorgrad::rainbow()`
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/rainbow.png)

`colorgrad::sinebow()`
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/sinebow.png)

## Hard-Edged Gradient

```rust
let g1 = colorgrad::CustomGradient::new()
    .html_colors(&["#18dbf4", "#f6ff56"])
    .build()
    .unwrap();

let g2 = g1.sharp(7);
```
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/gradient-normal.png)

![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/gradient-sharp.png)

```rust
let g = colorgrad::spectral().sharp(19);
```
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/spectral-sharp.png)

## Examples

### Gradient Image

```rust
extern crate colorgrad;
extern crate image;

fn main() {
    let grad = colorgrad::CustomGradient::new()
        .html_colors(&["deeppink", "gold", "seagreen"])
        .build()
        .unwrap();

    let w = 1500;
    let h = 70;
    let fw = w as f64;

    let mut imgbuf = image::ImageBuffer::new(w, h);

    for (x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
        let (r, g, b, _) = grad.at(x as f64 / fw).rgba_u8();
        *pixel = image::Rgb([r, g, b]);
    }

    imgbuf.save("gradient.png").unwrap();
}
```
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/example-gradient.png)

### Colored Noise

```rust
extern crate colorgrad;
extern crate image;
extern crate noise;

use noise::NoiseFn;

fn main() {
    let w = 600;
    let h = 350;
    let scale = 0.015;

    let grad = colorgrad::spectral();
    let ns = noise::OpenSimplex::new();
    let mut imgbuf = image::ImageBuffer::new(w, h);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let t = ns.get([x as f64 * scale, y as f64 * scale]);
        let (r, g, b, _) = grad.at(remap(t, -0.5, 0.5, 0.0, 1.0)).rgba_u8();
        *pixel = image::Rgb([r, g, b]);
    }
    imgbuf.save("noise.png").unwrap();
}

// Map value which is in range [a, b] to range [c, d]
fn remap(value: f64, a: f64, b: f64, c: f64, d: f64) -> f64 {
    (value - a) * ((d - c) / (b - a)) + c
}
```
![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/example-noise.png)

## Inspirations

* [chroma.js](https://github.com/gka/chroma.js)
* [d3-scale-chromatic](https://github.com/d3/d3-scale-chromatic/)

## Links

* [colorgrad](https://github.com/mazznoer/colorgrad) - Go version of this library

