use std::marker::PhantomData;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Into<wgpu::Color> for Color {
    fn into(self) -> wgpu::Color {
        let Color { r, g, b, a } = self;
        let (r, g, b, a) = (r as f64, g as f64, b as f64, a as f64);
        wgpu::Color { r, g, b, a }
    }
}

pub struct Canvas<'a, T: AsMut<[Color]> = &'a mut [Color]> {
    _marker: PhantomData<&'a ()>,
    pub array: T,
    pub width: u32,
    pub height: u32,
}

impl<'a> Canvas<'a> {
    pub fn from_texture_source(texture_source: &'a mut [Color], width: u32, height: u32) -> Self {
        Self {
            _marker: PhantomData,
            array: texture_source,
            width,
            height,
        }
    }

    pub fn load_from_file(
        path: impl AsRef<std::path::Path>,
    ) -> Result<Canvas<'a, Box<[Color]>>, crate::Error> {
        let image = image::open(path).map_err(crate::Error::ImageLoading)?;
        let image = image.to_rgba32f();
        let (width, height) = image.dimensions();
        let array = image
            .into_raw()
            .chunks_exact(4)
            .map(|rgba| Color {
                r: rgba[0],
                g: rgba[1],
                b: rgba[2],
                a: rgba[3],
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Ok(Canvas {
            _marker: PhantomData,
            array,
            width,
            height,
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = &Color> {
        self.array.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Color> {
        self.array.iter_mut()
    }

    pub fn get(&self, x: u32, y: u32) -> Option<&Color> {
        if x < self.width && y < self.height {
            Some(&self.array[(y * self.width + x) as usize])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, x: u32, y: u32) -> Option<&mut Color> {
        if x < self.width && y < self.height {
            Some(&mut self.array[(y * self.width + x) as usize])
        } else {
            None
        }
    }

    pub fn set(&mut self, x: u32, y: u32, color: Color) {
        if let Some(c) = self.get_mut(x, y) {
            *c = color;
        }
    }

    pub fn clear(&mut self, color: Color) {
        for c in self.array.iter_mut() {
            *c = color;
        }
    }

    pub fn fill(&mut self, x: u32, y: u32, width: u32, height: u32, color: Color) {
        for y in y..y + height {
            for x in x..x + width {
                self.set(x, y, color);
            }
        }
    }

    pub fn blit(&mut self, x: u32, y: u32, source: &Self) {
        for sy in 0..source.height {
            for sx in 0..source.width {
                if let Some(color) = source.get(x, y) {
                    self.set(x + sx, y + sy, *color);
                }
            }
        }
    }
}
