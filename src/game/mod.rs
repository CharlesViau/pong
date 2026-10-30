//! Bevy host: presentation and infrastructure adapters wired by composition.
mod composition;
mod infrastructure;
mod presentation;

#[cfg(test)]
mod tests;

pub use composition::PongPlugin;
