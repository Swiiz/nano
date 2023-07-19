use std::{collections::HashMap, path::Path};

use crate::Error;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const BLACK: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const WHITE: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const RED: Self = Self {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const GREEN: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    pub const BLUE: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
    pub const YELLOW: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    pub const CYAN: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const MAGENTA: Self = Self {
        r: 1.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };

    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub const fn new_alpha(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

impl Into<wgpu::Color> for Color {
    fn into(self) -> wgpu::Color {
        let Color { r, g, b, a } = self;
        let (r, g, b, a) = (r as f64, g as f64, b as f64, a as f64);
        wgpu::Color { r, g, b, a }
    }
}

pub struct Canvas {
    pub data: Vec<Color>,
    pub width: u32,
    pub height: u32,
}

impl Canvas {
    pub fn new(width: u32, height: u32, color: Color) -> Self {
        Self {
            data: vec![color; (width * height) as usize],
            width,
            height,
        }
    }

    pub fn load_from_file(path: impl AsRef<std::path::Path>) -> Result<Canvas, crate::Error> {
        let image = image::open(path).map_err(crate::Error::ImageLoading)?;
        let image = image.to_rgba32f();
        let (width, height) = image.dimensions();
        let data = image
            .into_raw()
            .chunks_exact(4)
            .map(|rgba| Color {
                r: rgba[0],
                g: rgba[1],
                b: rgba[2],
                a: rgba[3],
            })
            .collect::<Vec<_>>();
        Ok(Canvas {
            data,
            width,
            height,
        })
    }

    pub fn size_matches(&self, width: u32, height: u32) -> bool {
        assert!(self.data.len() == (self.width * self.height) as usize);
        self.width == width && self.height == height
    }

    pub fn iter(&self) -> impl Iterator<Item = &Color> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Color> {
        self.data.iter_mut()
    }

    pub fn get(&self, x: u32, y: u32) -> Option<&Color> {
        if x < self.width && y < self.height {
            Some(&self.data[(y * self.width + x) as usize])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, x: u32, y: u32) -> Option<&mut Color> {
        if x < self.width && y < self.height {
            Some(&mut self.data[(y * self.width + x) as usize])
        } else {
            None
        }
    }

    pub fn set(&mut self, x: u32, y: u32, color: Color) {
        if let Some(c) = self.get_mut(x, y) {
            *c = color;
        }
    }

    /// set method without checking if the pixel is in bounds, might panic!
    pub fn set_unchecked(&mut self, x: u32, y: u32, color: Color) {
        self.data[(y * self.width + x) as usize] = color;
    }

    pub fn clear(&mut self, color: Color) {
        self.data = vec![color; (self.width * self.height) as usize];
    }

    pub fn resize(&mut self, width: u32, height: u32, color: Color) {
        if self.size_matches(width, height) {
            return;
        }
        let mut new_data = vec![color; (width * height) as usize];
        for (row_idx, row) in self.data.chunks_exact(self.width as usize).enumerate() {
            for (i, c) in row.iter().enumerate() {
                if i < width as usize && row_idx < height as usize {
                    new_data[(row_idx * width as usize + i) as usize] = *c;
                }
            }
        }
        self.width = width;
        self.height = height;
        self.data = new_data;
    }

    pub fn fill(&mut self, x: u32, y: u32, width: u32, height: u32, color: Color) {
        for y in y..y + height {
            for x in x..x + width {
                self.set(x, y, color);
            }
        }
    }

    pub fn blit(&mut self, x: i32, y: i32, source: &Self, blend_mode: BlendMode) {
        for sy in 0..source.height {
            for sx in 0..source.width {
                if let Some(color) = source.get(sx, sy) {
                    let (x, y) = (x + sx as i32, y + sy as i32);
                    if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
                        let (x, y) = (x as u32, y as u32);
                        self.set_unchecked(
                            x,
                            y,
                            blend_mode.blend(*color, *self.get(x, y).unwrap()),
                        );
                    }
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum BlendMode {
    None,
    Alpha,
    Add,
}

impl BlendMode {
    pub fn blend(&self, first: Color, second: Color) -> Color {
        match self {
            BlendMode::None => first,
            BlendMode::Alpha => {
                let Color {
                    r: r1,
                    g: g1,
                    b: b1,
                    a: a1,
                } = first;
                let Color {
                    r: r2,
                    g: g2,
                    b: b2,
                    a: a2,
                } = second;
                let a = a1 + a2 * (1.0 - a1);
                let r = (r1 * a1 + r2 * a2 * (1.0 - a1)) / a;
                let g = (g1 * a1 + g2 * a2 * (1.0 - a1)) / a;
                let b = (b1 * a1 + b2 * a2 * (1.0 - a1)) / a;
                Color { r, g, b, a }
            }
            BlendMode::Add => {
                let Color {
                    r: r1,
                    g: g1,
                    b: b1,
                    a: a1,
                } = first;
                let Color {
                    r: r2,
                    g: g2,
                    b: b2,
                    a: a2,
                } = second;
                let a = a1 + a2;
                let r = (r1 * a1 + r2 * a2) / a;
                let g = (g1 * a1 + g2 * a2) / a;
                let b = (b1 * a1 + b2 * a2) / a;
                Color { r, g, b, a }
            }
        }
    }
}

pub struct CanvasLoader {
    loaded: HashMap<String, Canvas>,
}

fn path_ref_as_owned_str(path: impl AsRef<Path>) -> String {
    path.as_ref()
        .to_str()
        .expect("Path could not be decoded to string correctly")
        .to_owned()
}

impl CanvasLoader {
    pub fn new() -> Self {
        CanvasLoader {
            loaded: Default::default(),
        }
    }

    pub fn is_loaded(&self, path: impl AsRef<Path>) -> bool {
        self.loaded.contains_key(&path_ref_as_owned_str(path))
    }

    pub fn get_loaded(&self, path: impl AsRef<Path>) -> Option<&Canvas> {
        self.loaded.get(&path_ref_as_owned_str(path))
    }

    pub fn pre_load(&mut self, path: impl AsRef<Path>) -> Result<(), Error> {
        let path = path.as_ref();
        self.loaded
            .insert(path_ref_as_owned_str(path), Canvas::load_from_file(path)?);
        Ok(())
    }

    pub fn get_or_load(&mut self, path: impl AsRef<Path>) -> Result<&Canvas, Error> {
        let path = path.as_ref();
        if !self.is_loaded(path) {
            self.pre_load(path)?;
        }
        Ok(self.get_loaded(path).unwrap())
    }
}
