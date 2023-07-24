use std::rc::Rc;

use nano::*;
use photon::{renderers::scaling::ScalingRenderer2d, Canvas, Color};

fn main() -> Result<()> {
    nano::run::<ExampleWithPhoton>()
}

struct ExampleWithPhoton {
    window: Rc<nano::Window>,
    graphics: photon::Instance,
    scaling_renderer: ScalingRenderer2d,
}

const CANVAS_TO_SCREEN_RATIO: u32 = 10;

fn compute_scaled_size(window: &Window) -> (u32, u32) {
    let (width, height): (u32, u32) = window.inner_size().into();
    let (width, height) = (
        width / CANVAS_TO_SCREEN_RATIO,
        height / CANVAS_TO_SCREEN_RATIO,
    );
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
            nano::Event::RedrawRequested(_) => {
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

                self.graphics.render(|graphics, mut frame| {
                    let mut render_pass = frame.render_pass(Color::BLACK);
                    self.scaling_renderer.draw(graphics, &mut render_pass);
                    Ok(())
                })?;
            }
            nano::Event::MainEventsCleared => {
                self.window.request_redraw();
            }
            nano::Event::WindowEvent {
                event, window_id, ..
            } => {
                if *window_id == self.window.id() {
                    match event {
                        WindowEvent::Resized(size) => self.on_resize((*size).into()),
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            self.on_resize((**new_inner_size).into())
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
}

impl ExampleWithPhoton {
    fn on_resize(&mut self, new_size: (u32, u32)) {
        self.graphics.resize_surface(new_size);
        let (canvas_width, canvas_height) = compute_scaled_size(&self.window);
        self.scaling_renderer
            .resize_canvas(&self.graphics, canvas_width, canvas_height);
    }
}
