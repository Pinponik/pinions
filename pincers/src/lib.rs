#[cfg_attr(feature = "no_std", no_std)]
#[cfg(feature = "no_std")]
pub use heapless;
pub use pincers_macros;
pub use winit;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

#[cfg(feature = "no_std")]
type Str<const N: usize> = heapless::String<N>;

#[cfg(not(feature = "no_std"))]
type Str<const N: usize> = String;

#[cfg(feature = "no_std")]
type Vect<T, const N: usize> = heapless::Vec<T, N>;

#[cfg(not(feature = "no_std"))]
type Vect<T, const N: usize = 0> = Vec<T>;

/// A simple button widget with a fixed‑size label.
pub struct Button {
    pub label: Str<32>,
    // Additional fields (position, size, callback, etc.) can be added here.
}

impl Button {
    /// Creates a new button from a string slice.
    /// Returns `None` if the label does not fit into the fixed‑size buffer.
    pub fn new() -> Self {
        let mut lbl = Str::<32>::new();
        Self { label: lbl }
    }
}

pub struct Win<const T: usize, const E: usize, const V: usize> {
    window: Option<Window>,
    title: Str<T>,
    poll: bool,
    widgets: Vect<Button, V>,
}

impl<const T: usize, const E: usize, const V: usize> Win<T, E, V> {
    pub fn new() -> Self {
        Self {
            window: None,
            title: Str::<T>::new(),
            poll: false,
            widgets: Vect::<Button, V>::new(),
        }
    }

    pub fn default() -> Self {
        Self::new()
    }
}
