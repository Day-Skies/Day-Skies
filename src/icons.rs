//! Original weather glyphs drawn with the canvas display list (§11) — no bundled assets, so they
//! render identically on every backend and scale cleanly. Each `weather_icon` returns a square
//! `canvas` piece sized to `size`.

use crate::weather::Family;
use day::prelude::*;

/// Which glyph to draw. Derived from a `Family` plus day/night for clear & partly-cloudy skies.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Glyph {
    Sun,
    Moon,
    PartlyDay,
    PartlyNight,
    Cloud,
    Fog,
    Drizzle,
    Rain,
    Snow,
    Thunder,
}

impl Glyph {
    pub fn of(family: Family, is_day: bool) -> Glyph {
        match family {
            Family::Clear => {
                if is_day {
                    Glyph::Sun
                } else {
                    Glyph::Moon
                }
            }
            Family::PartlyCloudy => {
                if is_day {
                    Glyph::PartlyDay
                } else {
                    Glyph::PartlyNight
                }
            }
            Family::Cloudy => Glyph::Cloud,
            Family::Fog => Glyph::Fog,
            Family::Drizzle => Glyph::Drizzle,
            Family::Rain => Glyph::Rain,
            Family::Snow => Glyph::Snow,
            Family::Thunder => Glyph::Thunder,
        }
    }
}

// Palette (warm sun, pale moon, soft clouds, cool precipitation) — reads well on the sky tints.
const SUN: Color = Color::hex(0xFFD24A);
const SUN_CORE: Color = Color::hex(0xFFB300);
const MOON: Color = Color::hex(0xF3EDD2);
const CLOUD: Color = Color::hex(0xF2F6FB);
const CLOUD_DK: Color = Color::hex(0xCAD5E2);
const RAINDROP: Color = Color::hex(0x8FC7FF);
const SNOW: Color = Color::hex(0xFFFFFF);
const BOLT: Color = Color::hex(0xFFD24A);
const FOGLINE: Color = Color::hex(0xE3E9F0);

/// A weather glyph as a square canvas piece.
pub fn weather_icon(glyph: Glyph, size: f64) -> AnyPiece {
    canvas(move |d, area| {
        let s = area.width.min(area.height);
        if s <= 0.0 {
            return;
        }
        let ox = (area.width - s) / 2.0;
        let oy = (area.height - s) / 2.0;
        let p = |fx: f64, fy: f64| Point::new(ox + fx * s, oy + fy * s);
        // A rect in fractional [0,1] coordinates of the square.
        let r = |fx: f64, fy: f64, fw: f64, fh: f64| {
            Rect::new(ox + fx * s, oy + fy * s, fw * s, fh * s)
        };

        match glyph {
            Glyph::Sun => sun(d, p, r, s, 0.5, 0.5, 1.0),
            Glyph::Moon => moon(d, r),
            Glyph::Cloud => cloud(d, r(0.06, 0.20, 0.88, 0.60), CLOUD, CLOUD_DK),
            Glyph::PartlyDay => {
                sun(d, p, r, s, 0.34, 0.36, 0.62);
                cloud(d, r(0.24, 0.40, 0.70, 0.50), CLOUD, CLOUD_DK);
            }
            Glyph::PartlyNight => {
                d.fill(Shape::Ellipse(r(0.16, 0.14, 0.40, 0.40)), MOON);
                cloud(d, r(0.24, 0.40, 0.70, 0.50), CLOUD, CLOUD_DK);
            }
            Glyph::Fog => {
                cloud(d, r(0.06, 0.10, 0.88, 0.52), CLOUD, CLOUD_DK);
                for i in 0..3 {
                    let y = 0.72 + i as f64 * 0.11;
                    d.stroke(Shape::Line(p(0.16, y), p(0.84, y)), FOGLINE, s * 0.055);
                }
            }
            Glyph::Drizzle => {
                cloud(d, r(0.06, 0.12, 0.88, 0.54), CLOUD, CLOUD_DK);
                drops(d, p, s, 0.80, 0.92, RAINDROP);
            }
            Glyph::Rain => {
                cloud(d, r(0.06, 0.10, 0.88, 0.54), CLOUD, CLOUD_DK);
                drops(d, p, s, 0.78, 0.98, RAINDROP);
            }
            Glyph::Snow => {
                cloud(d, r(0.06, 0.10, 0.88, 0.54), CLOUD, CLOUD_DK);
                for (i, fx) in [0.30, 0.50, 0.70].into_iter().enumerate() {
                    let cy = if i == 1 { 0.90 } else { 0.84 };
                    d.fill(Shape::Ellipse(r(fx - 0.045, cy - 0.045, 0.09, 0.09)), SNOW);
                }
            }
            Glyph::Thunder => {
                cloud(d, r(0.06, 0.08, 0.88, 0.54), CLOUD, CLOUD_DK);
                let bolt = vec![
                    p(0.52, 0.66),
                    p(0.40, 0.90),
                    p(0.50, 0.90),
                    p(0.44, 1.02),
                    p(0.62, 0.80),
                    p(0.52, 0.80),
                    p(0.60, 0.66),
                ];
                d.fill(Shape::Polygon(bolt), BOLT);
            }
        }
    })
    .frame(size, size)
}

/// A sun centred at (cxf, cyf) with radius scaled by `scale`, plus rays.
fn sun<P, R>(d: &mut Draw, p: P, r: R, s: f64, cxf: f64, cyf: f64, scale: f64)
where
    P: Fn(f64, f64) -> Point,
    R: Fn(f64, f64, f64, f64) -> Rect,
{
    let rad = 0.20 * scale;
    // Rays.
    for k in 0..8 {
        let a = k as f64 * std::f64::consts::FRAC_PI_4;
        let (sa, ca) = a.sin_cos();
        let r1 = rad + 0.06 * scale;
        let r2 = rad + 0.15 * scale;
        d.stroke(
            Shape::Line(
                p(cxf + ca * r1, cyf + sa * r1),
                p(cxf + ca * r2, cyf + sa * r2),
            ),
            SUN,
            s * 0.045 * scale,
        );
    }
    d.fill(
        Shape::Ellipse(r(cxf - rad, cyf - rad, rad * 2.0, rad * 2.0)),
        SUN,
    );
    d.fill(
        Shape::Ellipse(r(
            cxf - rad * 0.62,
            cyf - rad * 0.62,
            rad * 1.24,
            rad * 1.24,
        )),
        SUN_CORE,
    );
}

/// A crescent-ish moon filling most of the square (two craters for character).
fn moon<R>(d: &mut Draw, r: R)
where
    R: Fn(f64, f64, f64, f64) -> Rect,
{
    d.fill(Shape::Ellipse(r(0.26, 0.16, 0.52, 0.52)), MOON);
    d.fill(Shape::Ellipse(r(0.40, 0.26, 0.08, 0.08)), CLOUD_DK);
    d.fill(Shape::Ellipse(r(0.55, 0.44, 0.06, 0.06)), CLOUD_DK);
}

/// A cloud within `b`: a rounded body plus three overlapping puffs, in two tones for depth.
fn cloud(d: &mut Draw, b: Rect, col: Color, shadow: Color) {
    let x = b.origin.x;
    let y = b.origin.y;
    let w = b.size.width;
    let h = b.size.height;
    // Soft shadow offset a touch down/right.
    d.fill(
        Shape::RoundedRect(
            Rect::new(x + 0.10 * w, y + 0.60 * h, 0.84 * w, 0.36 * h),
            0.18 * h,
        ),
        shadow,
    );
    // Body + puffs.
    d.fill(
        Shape::RoundedRect(
            Rect::new(x + 0.08 * w, y + 0.54 * h, 0.84 * w, 0.36 * h),
            0.18 * h,
        ),
        col,
    );
    d.fill(
        Shape::Ellipse(Rect::new(x + 0.06 * w, y + 0.36 * h, 0.42 * w, 0.42 * h)),
        col,
    );
    d.fill(
        Shape::Ellipse(Rect::new(x + 0.30 * w, y + 0.20 * h, 0.46 * w, 0.46 * h)),
        col,
    );
    d.fill(
        Shape::Ellipse(Rect::new(x + 0.52 * w, y + 0.34 * h, 0.40 * w, 0.40 * h)),
        col,
    );
}

/// Three slanted rain streaks between fractional y `top`…`bot`.
fn drops<P>(d: &mut Draw, p: P, s: f64, top: f64, bot: f64, col: Color)
where
    P: Fn(f64, f64) -> Point,
{
    for fx in [0.34, 0.52, 0.70] {
        d.stroke(Shape::Line(p(fx, top), p(fx - 0.05, bot)), col, s * 0.05);
    }
}
