use crate::{convert_colors, BlendMode, Color, GradientBase};

// Catmull-Rom spline algorithm adapted from:
// https://qroph.github.io/2018/07/30/smooth-paths-using-catmull-rom-splines.html

#[derive(Debug, Clone)]
pub(crate) struct CatmullRomGradient {
    segments: Vec<[[f64; 4]; 4]>,
    positions: Vec<f64>,
    domain: (f64, f64),
    mode: BlendMode,
    first_color: Color,
    last_color: Color,
}

fn to_catmull_segments(values: &[f64]) -> Vec<[f64; 4]> {
    let alpha = 0.5;
    let tension = 0.0;
    let n = values.len();

    let mut vals = Vec::with_capacity(n + 2);
    vals.push(2.0 * values[0] - values[1]);
    for v in values.iter() {
        vals.push(*v);
    }
    vals.push(2.0 * values[n - 1] - values[n - 2]);

    let mut segments = Vec::with_capacity(n - 1);

    for i in 1..(vals.len() - 2) {
        let v0 = vals[i - 1];
        let v1 = vals[i];
        let v2 = vals[i + 1];
        let v3 = vals[i + 2];

        let t0 = 0.0;
        let t1 = t0 + (v0 - v1).abs().powf(alpha);
        let t2 = t1 + (v1 - v2).abs().powf(alpha);
        let t3 = t2 + (v2 - v3).abs().powf(alpha);

        let m1 = (1.0 - tension)
            * (t2 - t1)
            * ((v0 - v1) / (t0 - t1) - (v0 - v2) / (t0 - t2) + (v1 - v2) / (t1 - t2));
        let m2 = (1.0 - tension)
            * (t2 - t1)
            * ((v1 - v2) / (t1 - t2) - (v1 - v3) / (t1 - t3) + (v2 - v3) / (t2 - t3));
        let m1 = if m1.is_nan() { 0.0 } else { m1 };
        let m2 = if m2.is_nan() { 0.0 } else { m2 };

        let a = 2.0 * v1 - 2.0 * v2 + m1 + m2;
        let b = -3.0 * v1 + 3.0 * v2 - 2.0 * m1 - m2;
        let c = m1;
        let d = v1;

        segments.push([a, b, c, d]);
    }
    segments
}

impl CatmullRomGradient {
    pub(crate) fn new(colors: Vec<Color>, positions: Vec<f64>, mode: BlendMode) -> Self {
        let n = colors.len();
        let mut a = Vec::with_capacity(n);
        let mut b = Vec::with_capacity(n);
        let mut c = Vec::with_capacity(n);
        let mut d = Vec::with_capacity(n);

        for col in convert_colors(&colors, mode) {
            a.push(col[0]);
            b.push(col[1]);
            c.push(col[2]);
            d.push(col[3]);
        }

        let s1 = to_catmull_segments(&a);
        let s2 = to_catmull_segments(&b);
        let s3 = to_catmull_segments(&c);
        let s4 = to_catmull_segments(&d);

        let dmin = positions[0];
        let dmax = positions[positions.len() - 1];
        let first_color = colors[0].clone();
        let last_color = colors[colors.len() - 1].clone();

        Self {
            segments: s1
                .iter()
                .zip(&s2)
                .zip(&s3)
                .zip(&s4)
                .map(|(((a, b), c), d)| [*a, *b, *c, *d])
                .collect(),
            positions,
            domain: (dmin, dmax),
            mode,
            first_color,
            last_color,
        }
    }
}

impl GradientBase for CatmullRomGradient {
    fn at(&self, t: f64) -> Color {
        if t <= self.domain.0 {
            return self.first_color.clone();
        }

        if t >= self.domain.1 {
            return self.last_color.clone();
        }

        for (pos, [seg_a, seg_b, seg_c, seg_d]) in self.positions.windows(2).zip(&self.segments) {
            if (pos[0] <= t) && (t <= pos[1]) {
                let t1 = (t - pos[0]) / (pos[1] - pos[0]);
                let t2 = t1 * t1;
                let t3 = t2 * t1;

                let c0 = seg_a[0] * t3 + seg_a[1] * t2 + seg_a[2] * t1 + seg_a[3];
                let c1 = seg_b[0] * t3 + seg_b[1] * t2 + seg_b[2] * t1 + seg_b[3];
                let c2 = seg_c[0] * t3 + seg_c[1] * t2 + seg_c[2] * t1 + seg_c[3];
                let c3 = seg_d[0] * t3 + seg_d[1] * t2 + seg_d[2] * t1 + seg_d[3];

                match self.mode {
                    BlendMode::LinearRgb => return Color::from_linear_rgba(c0, c1, c2, c3),
                    BlendMode::Oklab => return Color::from_oklaba(c0, c1, c2, c3),
                    _ => return Color::new(c0, c1, c2, c3),
                }
            }
        }

        Color::new(0.0, 0.0, 0.0, 1.0)
    }
}