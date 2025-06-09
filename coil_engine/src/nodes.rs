use crate::renderer::Renderer;
use crossterm::event::Event;

mod container;
pub use container::Container;

pub trait Node {
    /// Called once per fixed‐timestep tick
    fn update(&mut self, dt: f32);

    /// Called for each input event; return `true` to consume it
    fn on_event(&mut self, ev: Event) -> bool;

    /// Draw yourself into the given renderer.  Children drawn automatically.
    fn render(&self, r: &mut dyn Renderer);
}
