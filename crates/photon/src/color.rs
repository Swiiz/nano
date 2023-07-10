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

pub struct ColorArray<'a> {
    pub array: &'a mut [Color],
    pub width: u32,
    pub height: u32,
}

impl<'a> ColorArray<'a> {
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
}
