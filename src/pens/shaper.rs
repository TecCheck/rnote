use serde::{Deserialize, Serialize};

use crate::strokes;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CurrentShape {
    Rectangle,
    Ellipse,
}

impl Default for CurrentShape {
    fn default() -> Self {
        Self::Ellipse
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LineConfig {
    width: f64,
    color: Option<strokes::Color>,
    fill: Option<strokes::Color>,
}

impl Default for LineConfig {
    fn default() -> Self {
        Self {
            width: Self::WIDTH_DEFAULT,
            color: Self::COLOR_DEFAULT,
            fill: Self::FILL_DEFAULT,
        }
    }
}

impl LineConfig {
    pub const WIDTH_MIN: f64 = 1.0;
    pub const WIDTH_MAX: f64 = 500.0;
    pub const WIDTH_DEFAULT: f64 = 5.0;
    pub const COLOR_DEFAULT: Option<strokes::Color> = Some(strokes::Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    });
    pub const FILL_DEFAULT: Option<strokes::Color> = None;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RectangleConfig {
    width: f64,
    color: Option<strokes::Color>,
    fill: Option<strokes::Color>,
}

impl Default for RectangleConfig {
    fn default() -> Self {
        Self {
            width: Self::WIDTH_DEFAULT,
            color: Self::COLOR_DEFAULT,
            fill: Self::FILL_DEFAULT,
        }
    }
}

impl RectangleConfig {
    pub const WIDTH_MIN: f64 = 1.0;
    pub const WIDTH_MAX: f64 = 500.0;
    pub const WIDTH_DEFAULT: f64 = 5.0;
    pub const COLOR_DEFAULT: Option<strokes::Color> = Some(strokes::Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    });
    pub const FILL_DEFAULT: Option<strokes::Color> = None;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EllipseConfig {
    width: f64,
    color: Option<strokes::Color>,
    fill: Option<strokes::Color>,
}

impl Default for EllipseConfig {
    fn default() -> Self {
        Self {
            width: Self::WIDTH_DEFAULT,
            color: Self::COLOR_DEFAULT,
            fill: Self::FILL_DEFAULT,
        }
    }
}

impl EllipseConfig {
    pub const WIDTH_MIN: f64 = 1.0;
    pub const WIDTH_MAX: f64 = 500.0;
    pub const WIDTH_DEFAULT: f64 = 5.0;
    pub const COLOR_DEFAULT: Option<strokes::Color> = Some(strokes::Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    });
    pub const FILL_DEFAULT: Option<strokes::Color> = None;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Shaper {
    width: f64,
    pub color: strokes::Color,
    pub current_shape: CurrentShape,
    pub line_config: LineConfig,
    pub rectangle_config: RectangleConfig,
    pub ellipse_config: EllipseConfig,
}

impl Default for Shaper {
    fn default() -> Self {
        Self {
            width: Self::WIDTH_DEFAULT,
            color: Self::COLOR_DEFAULT,
            current_shape: CurrentShape::default(),
            line_config: LineConfig::default(),
            rectangle_config: RectangleConfig::default(),
            ellipse_config: EllipseConfig::default(),
        }
    }
}

impl Shaper {
    pub const WIDTH_MIN: f64 = 1.0;
    pub const WIDTH_MAX: f64 = 500.0;
    pub const WIDTH_DEFAULT: f64 = 5.0;

    pub const COLOR_DEFAULT: strokes::Color = strokes::Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };

    pub fn width(&self) -> f64 {
        self.width
    }

    pub fn set_width(&mut self, width: f64) {
        self.width = width.clamp(Self::WIDTH_MIN, Self::WIDTH_MAX);
    }
}