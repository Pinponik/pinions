//!                          ____  _       _
//!                         / __ \(_)___  (_)___  ____  _____
//!                        / /_/ / / __ \/ / __ \/ __ \/ ___/
//!                       / ____/ / / / / / /_/ / / / (__  )
//!                      /_/   /_/_/ /_/_/\____/_/ /_/____/
//!
//!           A fast and easy-to-use GUI library running on microcontrolleres

#[cfg(feature = "no_std")]
use core::fmt::{Debug, Write};
#[cfg_attr(feature = "no_std", no_std)]
#[cfg(feature = "no_std")]
pub use heapless;
pub use num;
pub use num::Num;
pub use pinions_macros;
#[cfg(feature = "debug")]
use std::any::type_name;
#[cfg(not(feature = "no_std"))]
use std::fmt::{Debug, Write};
use std::sync::{Arc, Mutex};
use std::task::Poll;
use std::time::Instant;
pub use winit;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

#[cfg(feature = "no_std")]
pub type Str<const N: usize> = heapless::String<N>;

#[cfg(not(feature = "no_std"))]
pub type Str<const N: usize> = String;

#[cfg(feature = "no_std")]
pub type Vect<T, const N: usize> = heapless::Vec<T, N>;

#[cfg(not(feature = "no_std"))]
pub type Vect<T, const N: usize = 0> = Vec<T>;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point {
    x: f32,
    y: f32,
}

pub struct Mouse {
    position: Option<Point>,
    pressed: bool,
}

pub struct Event {
    timestamp: Instant,
    event: isize,
}

impl Clone for Event {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for Event {}

impl Debug for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Event")
            .field("timestamp", &self.timestamp)
            .field("event", &self.event)
            .finish()
    }
}

impl Default for Event {
    fn default() -> Self {
        #[cfg(feature = "debug")]
        eprintln!("Ran Event::default()");
        Self {
            timestamp: Instant::now(),
            event: 0,
        }
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp.eq(&other.timestamp)
    }
}

impl Eq for Event {}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.timestamp.cmp(&other.timestamp))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

#[cfg(not(feature = "no_std"))]
pub type WinEvent = winit::event::WindowEvent;

pub trait PinionsApp {
    fn update<const E: usize>(&mut self, _events: Arc<Mutex<Vect<Event, E>>>, _event: WinEvent) {}
}

pub struct Wid<const L: usize, I: Num, const S: usize, const E: usize> {
    pub label: Str<L>,
    pub icon: Vect<I, S>,
    pub mouse: Mouse,
    pub events: Arc<Mutex<Vect<Event, E>>>,
}

impl<const L: usize, I: Num, const S: usize, const E: usize> Wid<L, I, S, E> {
    pub fn new() -> Self {
        #[cfg(feature = "debug")]
        {
            let i = type_name::<I>();
            eprintln!("Ran Wid::<L: {}, I: {}, S: {}, E: {}>::new()", L, i, S, E);
        }
        let mut lbl = Str::<L>::new();
        let icon = Vect::<I, S>::new();
        Self {
            label: lbl,
            icon,
            mouse: Mouse {
                position: None,
                pressed: false,
            },
            events: Arc::new(Mutex::new(Vect::<Event, E>::new())),
        }
    }

    pub fn sort_events(&self) {
        let mut events = self.events.lock().unwrap();
        events.as_mut_slice().sort();
    }
}

pub struct Win<
    const T: usize, // title length
    const E: usize, // event length
    const V: usize, // widget count
    const L: usize, // label length -\
    I: Num,         // icon type      | <- for struct Wid
    const S: usize, // icon size    -/
    A,
> {
    window: Option<Window>,
    title: Str<T>,
    count: u32,
    poll: bool,
    needs_redraw: bool,
    events: Arc<Mutex<Vect<Event, E>>>,
    widgets: Vect<Wid<L, I, S, E>, V>,
    app: A,
}

impl<const T: usize, const E: usize, const V: usize, const L: usize, I: Num, const S: usize, A>
    Win<T, E, V, L, I, S, A>
{
    pub fn new(app: A) -> Self {
        #[cfg(feature = "debug")]
        {
            let i = type_name::<I>();
            eprintln!(
                "Ran Win::<T: {}, E: {}, V: {}, L: {}, I: {}, S: {}>::new()",
                T, E, V, L, i, S
            );
        }
        Self {
            window: None,
            title: Str::<T>::new(),
            count: 0,
            poll: false,
            needs_redraw: false,
            events: Arc::new(Mutex::new(Vect::<Event, E>::new())),
            widgets: Vect::<Wid<L, I, S, E>, V>::new(),
            app: app,
        }
    }

    pub fn default(app: A) -> Self {
        Self::new(app)
    }
    pub fn title(&mut self, title: Str<T>) {
        self.title = title;
    }
}

impl<const T: usize, const E: usize, const V: usize, const L: usize, I: Num, const S: usize, A>
    winit::application::ApplicationHandler for Win<T, E, V, L, I, S, A>
{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        #[cfg(feature = "debug")]
        {
            let i = type_name::<I>();
            eprintln!(
                "Ran Win::<T: {}, E: {}, V: {}, L: {}, I: {}, S: {}>::resumed()",
                T, E, V, L, i, S
            );
        }
        if self.window.is_none() {
            let window_attributes =
                winit::window::Window::default_attributes().with_title(self.title.as_str());
            match event_loop.create_window(window_attributes) {
                Ok(window) => {
                    self.window = Some(window);
                }
                Err(err) => {
                    eprintln!("Failed to create window: {err}");
                    event_loop.exit();
                }
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        #[cfg(feature = "debug")]
        {
            let i = type_name::<I>();
            eprintln!(
                "Ran Win::<T: {}, E: {}, V: {}, L: {}, I: {}, S: {}>::window_event()",
                T, E, V, L, i, S
            );
        }
        match event {
            winit::event::WindowEvent::CursorMoved { position, .. } => {
                let point = Point {
                    x: position.x as f32,
                    y: position.y as f32,
                };
                for widget in self.widgets.iter_mut() {
                    let point_for_widget = point.clone();
                    widget.mouse.position = Some(point_for_widget);
                }
            }
            winit::event::WindowEvent::MouseInput { state, button, .. } => {
                if button == winit::event::MouseButton::Left {
                    let pressed = matches!(state, winit::event::ElementState::Pressed);
                    for widget in self.widgets.iter_mut() {
                        widget.mouse.pressed = pressed;
                    }
                    if pressed {
                        // Increment counter and update the first widget's label
                        self.count = self.count.wrapping_add(1);
                        if let Some(widget) = self.widgets.get_mut(0) {
                            widget.label.clear();
                            let _ = write!(widget.label, "{}", self.count);
                        }
                        // Request a redraw
                        self.needs_redraw = true;
                    }
                }
            }
            winit::event::WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        #[cfg(feature = "debug")]
        {
            let i = type_name::<I>();
            eprintln!(
                "Ran Win::<T: {}, E: {}, V: {}, L: {}, I: {}, S: {}>::about_to_wait()",
                T, E, V, L, i, S
            );
        }
        // Poll for events if needed
        if self.poll {
            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }
        // Redraw if needed
        if self.needs_redraw {
            if let Some(window) = &self.window {
                window.request_redraw();
            }
            self.needs_redraw = false;
        }
    }
}

impl<const T: usize, const E: usize, const V: usize, const L: usize, I: Num, const S: usize, A>
    Win<T, E, V, L, I, S, A>
{
    /// Start the event loop and run the application.
    /// This method will block until the event loop exits.
    pub fn run(mut self) -> Result<(), winit::error::EventLoopError> {
        #[cfg(feature = "debug")]
        {
            let i = type_name::<I>();
            let a = type_name::<A>();
            eprintln!(
                "Ran Win::<T: {}, E: {}, V: {}, L: {}, I: {}, S: {}, A: {}>::run()",
                T, E, V, L, i, S, a
            );
        }
        let event_loop = winit::event_loop::EventLoop::new()?;
        event_loop.set_control_flow(if self.poll {
            winit::event_loop::ControlFlow::Poll
        } else {
            if !self.events.lock().unwrap().is_empty() {
                winit::event_loop::ControlFlow::Wait
            } else {
                let instant = self.events.lock().unwrap()[0].timestamp.clone();
                winit::event_loop::ControlFlow::WaitUntil(instant)
            }
        });
        event_loop.run_app(&mut self)
    }

    pub fn sort_events(&self) {
        #[cfg(feature = "debug")]
        {
            let i = type_name::<I>();
            let a = type_name::<A>();
            eprintln!(
                "Ran Win::<T: {}, E: {}, V: {}, L: {}, I: {}, S: {}, A: {}>::sort_events()",
                T, E, V, L, i, S, a
            );
        }
        let mut events = self.events.lock().unwrap();
        events.as_mut_slice().sort();
    }

    pub fn poll(&mut self, poll: bool) {
        #[cfg(feature = "debug")]
        {
            let i = type_name::<I>();
            let a = type_name::<A>();
            eprintln!(
                "Ran set_poll::<T: {}, E: {}, V: {}, L: {}, I: {}, S: {}, A: {}>(poll: {})",
                T, E, V, L, i, S, a, poll
            );
        }
        self.poll = poll;
    }
}
