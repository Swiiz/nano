use std::rc::Rc;

use nano::*;
use photon::{
    renderers::{ScalingRenderer2d, ScalingRenderer2dConfig},
    Color,
};

fn main() -> Result<()> {
    nano::run::<ExampleWithPhoton>()
}

struct ExampleWithPhoton {
    window: Rc<nano::Window>,
    graphics: photon::Instance,
    scaling_renderer: ScalingRenderer2d,
}

impl nano::Game for ExampleWithPhoton {
    fn new(host: &nano::Host) -> nano::Result<Self> {
        let window = Rc::new(host.create_window(|wb| wb)?);
        let graphics = photon::Instance::new(window.clone())?;
        let scaling_renderer = ScalingRenderer2d::new(
            &graphics,
            ScalingRenderer2dConfig {
                background_color: Color {
                    r: 0.025,
                    g: 0.025,
                    b: 0.025,
                    a: 1.0,
                },
                upsampling_ratio: 10,
                texture_source: None,
            },
        )?;

        Ok(Self {
            window,
            graphics,
            scaling_renderer,
        })
    }

    fn on(&mut self, context: Context, event: nano::Event) -> nano::Result<()> {
        match event {
            nano::Event::Draw => {
                let mut pixels = self.scaling_renderer.get_color_array(&self.graphics);
                let mut i = 0;
                // Draw gradient lines
                for Color { r, g, b, a: _ } in pixels.iter_mut() {
                    i = (i + 1) % 255;
                    *r = (i) as f32 / 255.0;
                    *g = ((i + 75) % 255) as f32 / 255.0;
                    *b = ((i + 150) % 255) as f32 / 255.0;
                }
                // Draw a red square
                pixels.fill(
                    0,
                    0,
                    10,
                    10,
                    Color {
                        r: 1.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    },
                );

                self.graphics.render(|ctx, frame| {
                    self.scaling_renderer.draw(ctx, frame.encoder, frame.view)
                })?;
            }
            nano::Event::Update => {
                self.window.request_redraw();
            }
            nano::Event::CloseRequested { window_id } => {
                if window_id == self.window.id() {
                    context.control_flow.set_exit();
                }
            }
            nano::Event::WindowResize {
                window_id,
                new_size,
            } => {
                if window_id == self.window.id() {
                    self.graphics.resize_surface(new_size);
                }
            }
        }
        Ok(())
    }
}
