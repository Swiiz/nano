use std::collections::HashSet;

use winit::event::WindowEvent;

#[derive(Default)]
pub struct Inputs {
    pub keyboard: Keyboard,
}

impl Inputs {
    pub(crate) fn handle(&mut self, event: &winit::event::Event<()>) {
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput { input, .. } => {
                    let Some(key) = input.virtual_keycode else {
                        return;
                    };
                    match input.state {
                        winit::event::ElementState::Pressed => {
                            self.keyboard.key_pressed.insert(key);
                        }
                        winit::event::ElementState::Released => {
                            self.keyboard.key_pressed.remove(&key);
                        }
                    };
                }
                _ => {}
            },
            _ => {}
        }
    }
}

pub type Key = winit::event::VirtualKeyCode;

#[derive(Default)]
pub struct Keyboard {
    pub key_pressed: HashSet<Key>,
}

impl Keyboard {
    pub fn is_pressed(&self, key: Key) -> bool {
        self.key_pressed.contains(&key)
    }
}
