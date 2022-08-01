use crate::{convert_colors, BlendMode, Color, GradientBase};

// Basis spline algorithm adapted from:
// https://github.com/d3/d3-interpolate/blob/master/src/basis.js

#[inline]
fn basis(t1: f64, v0: f64, v1: f64, v2: f64, v3: f64) -> f64 {
    let t2 = t1 * t1;
    let t3 = t2 * t1;
    ((1.0 - 3.0 * t1 + 3.0 * t2 - t3) * v0
        + (4.0 - 6.0 * t2 + 3.0 * t3) * v1
        + (1.0 + 3.0 * t1 + 3.0 * t2 - 3.0 * t3) * v2
        + t3 * v3)
        / 6.0
}

#[derive(Debug, Clone)]
pub(crate) struct BasisGradient {
    values: Vec<[f64; 4]>,
    positions: Vec<f64>,
    domain: (f64, f64),
    mode: BlendMode,
    first_color: Color,
    last_color: Color,
}

impl BasisGradient {
    pub(crate) fn new(colors: Vec<Color>, positions: Vec<f64>, mode: BlendMode) -> Self {
        let dmin = positions[0];
        let dmax = positions[positions.len() - 1];
        let first_color = colors[0].clone();
        let last_color = colors[colors.len() - 1].clone();
        Self {
            values: convert_colors(&colors, mode),
            positions,
            domain: (dmin, dmax),
            mode,
            first_color,
            last_color,
        }
    }
}

impl GradientBase for BasisGradient {
    fn at(&self, t: f64) -> Color {
        if t <= self.domain.0 {
            return self.first_color.clone();
        }

        if t >= self.domain.1 {
            return self.last_color.clone();
        }

        let n = self.values.len() - 1;

        for (i, (pos, val)) in self
            .positions
            .windows(2)
            .zip(self.values.windows(2))
            .enumerate()
        {
            if (pos[0] <= t) && (t <= pos[1]) {
                let t = (t - pos[0]) / (pos[1] - pos[0]);
                let mut zz = [0.0; 4];

                for (j, (v1, v2)) in val[0].iter().zip(val[1].iter()).enumerate() {
                    let v0 = if i > 0 {
                        self.values[i - 1][j]
                    } else {
                        2.0 * v1 - v2
                    };

                    let v3 = if i < (n - 1) {
                        self.values[i + 2][j]
                    } else {
                        2.0 * v2 - v1
                    };

                    zz[j] = basis(t, v0, *v1, *v2, v3);
                }
                let [c0, c1, c2, c3] = zz;

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