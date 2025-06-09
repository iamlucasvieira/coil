use crate::nodes::Node;
use crate::renderer::Renderer;
use crossterm::event::Event;

/// A container node that can hold multiple child nodes.
pub struct Container {
    pub x: u16,
    pub y: u16,
    pub children: Vec<Box<dyn Node>>,
}

impl Node for Container {
    fn update(&mut self, dt: f32) {
        for c in &mut self.children {
            c.update(dt);
        }
    }
    fn on_event(&mut self, ev: Event) -> bool {
        // first give children a chance
        for c in self.children.iter_mut().rev() {
            if c.on_event(ev.clone()) {
                return true;
            }
        }
        false
    }
    fn render(&self, r: &mut dyn Renderer) {
        // push a translation, if you want…
        for c in &self.children {
            c.render(r);
        }
    }
}

impl Container {
    pub fn new(x: u16, y: u16) -> Self {
        Container {
            x,
            y,
            children: Vec::new(),
        }
    }

    /// Pushes a child and returns `self`, so you can chain.
    pub fn with_child<N: Node + 'static>(mut self, child: N) -> Self {
        self.children.push(Box::new(child));
        self
    }
}
