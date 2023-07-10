use std::rc::Rc;

use nano::*;
use photon::{renderers::ScalingRenderer2d, Canvas, Color};

fn main() -> Result<()> {
    nano::run::<ExampleWithPhoton>()
}

struct ExampleWithPhoton {
    window: Rc<nano::Window>,
    graphics: photon::Instance,
    scaling_renderer: ScalingRenderer2d,
}

const DOWNSAMPLING: u32 = 10;

fn compute_scaled_size(window: &Window) -> (u32, u32) {
    let (width, height): (u32, u32) = window.inner_size().into();
    let (width, height) = (width / DOWNSAMPLING, height / DOWNSAMPLING);
    (width, height)
}

impl nano::Game for ExampleWithPhoton {
    fn new(host: &nano::Host) -> nano::Result<Self> {
        let window = Rc::new(host.create_window(|wb| wb)?);
        let graphics = photon::Instance::new(window.clone())?;
        let (canvas_width, canvas_height) = compute_scaled_size(&window);
        let scaling_renderer = ScalingRenderer2d::new(
            &graphics,
            Canvas::new(canvas_width, canvas_height, Color::CYAN),
            Color::BLACK,
        )?;

        Ok(Self {
            window,
            graphics,
            scaling_renderer,
        })
    }

    fn on(&mut self, context: Context, event: &nano::Event) -> nano::Result<()> {
        match event {
            nano::Event::Draw => {
                let canvas = &mut self.scaling_renderer.canvas;
                let mut i = context.start.elapsed().as_millis() / 50;
                // Draw gradient lines
                for Color { r, g, b, a: _ } in canvas.iter_mut() {
                    i = (i + 1) % 255;
                    *r = (i) as f32 / 255.0;
                    *g = ((i + 75) % 255) as f32 / 255.0;
                    *b = ((i + 150) % 255) as f32 / 255.0;
                }
                // Draw a yellow square
                canvas.fill(0, 0, 10, 10, Color::YELLOW);

                self.graphics.render(|graphics, frame| {
                    self.scaling_renderer
                        .draw(graphics, frame.encoder, frame.view)
                })?;
            }
            nano::Event::Update => {
                self.window.request_redraw();
            }
            nano::Event::CloseRequested { window_id } => {
                if window_id == &self.window.id() {
                    context.control_flow.set_exit();
                }
            }
            nano::Event::WindowResize {
                window_id,
                new_size,
            } => {
                if window_id == &self.window.id() {
                    self.graphics.resize_surface(*new_size);
                    let (canvas_width, canvas_height) = compute_scaled_size(&self.window);
                    self.scaling_renderer.resize_canvas(
                        &self.graphics,
                        canvas_width,
                        canvas_height,
                    );
                }
            }
        }
        Ok(())
    }
}
