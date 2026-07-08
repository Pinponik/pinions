#!/                          ____  _       _
//                         / __ \(_)___  (_)___  ____  _____
//                        / /_/ / / __ \/ / __ \/ __ \/ ___/
//                       / ____/ / / / / /_/ / / / (__  )
//                      /_/   /_/_/ /_/_/\____/_/ /_/____/
//!
//!           A fast and easy-to-use GUI library running on microcontrolleres

#![cfg_attr(feature = "no_std", no_std)]

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

#[cfg(not(feature = "no_std"))]
pub type CallbackEvent = winit::event::WindowEvent;
#[cfg(feature = "no_std")]
pub type CallbackEvent = ();

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point {
    x: f32,
    y: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
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

pub struct Wid<const L: usize, I: Num, const S: usize, const E: usize, A> {
    pub label: Str<L>,
    pub label_closure: Option<Box<dyn FnMut(&mut A) -> Str<L> + 'static>>,
    pub icon: Vect<I, S>,
    pub mouse: Mouse,
    pub events: Arc<Mutex<Vect<Event, E>>>,
    pub callback: Box<
        dyn FnMut(
                &mut Self,
                &mut Win<L, I, S, E, A>,
                &mut A,
                Arc<Mutex<Vect<Event, E>>>,
                CallbackEvent,
            ) + 'static,
    >,
    pub rect: Option<Rect>,
}

impl<const L: usize, I: Num, const S: usize, const E: usize, A> Wid<L, I, S, E, A> {
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
            label_closure: None,
            icon,
            mouse: Mouse {
                position: None,
                pressed: false,
            },
            events: Arc::new(Mutex::new(Vect::<Event, E>::new())),
            callback: Box::new(|_, _, _, _| {}),
            rect: None,
        }
    }

    pub fn label<F>(mut self, f: F) -> &mut Self
    where
        F: FnMut(&mut A) -> Str<L> + 'static,
    {
        self.label_closure = Some(Box::new(f));
        self
    }

    pub fn fun<F>(&mut self, f: F) -> &mut Self
    where
        F: FnMut(&mut Self, &mut A, Arc<Mutex<Vect<Event, E>>>, CallbackEvent) + 'static,
    {
        self.callback = Box::new(f);
        self
    }

    pub fn update_label(&mut self, app: &mut A) {
        if let Some(ref mut f) = self.label_closure {
            self.label = f(app);
        }
    }

    pub fn sort_events(&self) {
        let mut events = self.events.lock().unwrap();
        events.as_mut_slice().sort();
    }
}

pub struct Win<
    const T: usize, // title length
    const L: usize, // label length
    I: Num,         // icon type
    const S: usize, // icon size
    const E: usize, // event length
    A,
> {
    window: Option<Window>,
    title: Str<T>,
    title_closure: Option<Box<dyn FnMut(&mut A) -> Str<T> + 'static>>,
    poll: bool,
    needs_redraw: bool,
    events: Arc<Mutex<Vect<Event, E>>>,
    window_size: Option<(u32, u32)>, // width, height
    mouse_position: Option<Point>,   // Cached mouse position
    widgets: Vec<Wid<L, I, S, E, A>>,
    app: A,
}

impl<const T: usize, const L: usize, I: Num, const S: usize, const E: usize, A>
    Win<T, L, I, S, E, A>
{
    pub fn new(app: A) -> Self {
        #[cfg(feature = "debug")]
        {
            let i = type_name::<I>();
            let a = type_name::<A>();
            eprintln!(
                "Ran Win::<T: {}, L: {}, I: {}, S: {}, E: {}, A: {}>::new()",
                T, L, i, S, E, a
            );
        }
        Self {
            window: None,
            title: Str::<T>::new(),
            title_closure: None,
            poll: false,
            needs_redraw: false,
            events: Arc::new(Mutex::new(Vect::<Event, E>::new())),
            window_size: None,
            mouse_position: None,
            widgets: Vec::new(),
            app,
        }
    }

    pub fn title<F>(mut self, f: F) -> Self
    where
        F: FnMut(&mut A) -> Str<T> + 'static,
    {
        self.title_closure = Some(Box::new(f));
        self
    }

    pub fn update_title(&mut self) {
        if let Some(ref mut f) = self.title_closure {
            self.title = f(&mut self.app);
        }
    }

    pub fn widget(&mut self, widgets: Vec<Wid<L, I, S, E, A>>) -> &mut Self {
        self.widgets = widgets;
        self
    }

    pub fn layout(&mut self) {
        if let Some((width, height)) = self.window_size {
            let padding = 10;
            let widget_height = 40;
            let spacing = 10;
            let total_height = (widget_height + spacing) * self.widgets.len() as u32 - spacing;
            let start_y = (height as i32 - total_height as i32) / 2;

            for (i, widget) in self.widgets.iter_mut().enumerate() {
                let y = start_y + i as i32 * (widget_height + spacing) as i32;
                widget.rect = Some(Rect {
                    x: padding,
                    y,
                    width: (width as i32 - 2 * padding) as u32,
                    height: widget_height,
                });
            }
        }
    }
}

impl<const T: usize, const L: usize, I: Num, const S: usize, const E: usize, A>
    winit::application::ApplicationHandler for Win<T, L, I, S, E, A>
{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        #[cfg(feature = "debug")]
        {
            let i = type_name::<I>();
            let a = type_name::<A>();
            eprintln!(
                "Ran Win::<T: {}, L: {}, I: {}, S: {}, E: {}, A: {}>::resumed()",
                T, L, i, S, E, a
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
            let a = type_name::<A>();
            eprintln!(
                "Ran Win::<T: {}, L: {}, I: {}, S: {}, E: {}, A: {}>::window_event()",
                T, L, i, S, E, a
            );
        }
        match event {
            winit::event::WindowEvent::Resized(size) => {
                self.window_size = Some((size.width, size.height));
                self.needs_redraw = true;
                self.layout();
            }
            winit::event::WindowEvent::CursorMoved { position, .. } => {
                let point = Point {
                    x: position.x as f32,
                    y: position.y as f32,
                };
                self.mouse_position = Some(point);
                for widget in self.widgets.iter_mut() {
                    let point_for_widget = point.clone();
                    widget.mouse.position = Some(point_for_widget);
                }
            }
            winit::event::WindowEvent::MouseInput { state, button, .. } => {
                if button == winit::event::MouseButton::Left {
                    let pressed = matches!(state, winit::event::ElementState::Pressed);
                    // Update mouse state for all widgets
                    for widget in self.widgets.iter_mut() {
                        widget.mouse.pressed = pressed;
                    }
                    if pressed {
                        // Left button pressed: check for click on widgets
                        if let Some(pos) = self.mouse_position {
                            let (x, y) = (pos.x as i32, pos.y as i32);
                            for widget in self.widgets.iter_mut() {
                                if let Some(rect) = widget.rect {
                                    if x >= rect.x
                                        && x < (rect.x + rect.width as i32)
                                        && y >= rect.y
                                        && y < (rect.y + rect.height as i32)
                                    {
                                        // Found the widget that was clicked
                                        // Call the callback with four arguments
                                        (widget.callback)(
                                            widget,
                                            &mut self.app,
                                            self.events.clone(),
                                            event.clone(),
                                        );
                                    }
                                }
                            }
                        }
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
            let a = type_name::<A>();
            eprintln!(
                "Ran Win::<T: {}, L: {}, I: {}, S: {}, E: {}, A: {}>::about_to_wait()",
                T, L, i, S, E, a
            );
        }
        // Poll for events if needed
        if self.poll {
            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }
        // Update title
        self.update_title();
        // Update labels for all widgets
        for widget in &mut self.widgets {
            widget.update_label(&mut self.app);
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

impl<const T: usize, const L: usize, I: Num, const S: usize, const E: usize, A>
    Win<T, L, I, S, E, A>
{
    /// Start the event loop and run the application.
    /// This method will block until the event loop exits.
    pub fn run(mut self) -> Result<(), winit::error::EventLoopError> {
        #[cfg(feature = "debug")]
        {
            let i = type_name::<I>();
            let a = type_name::<A>();
            eprintln!(
                "Ran Win::<T: {}, L: {}, I: {}, S: {}, E: {}, A: {}>::run()",
                T, L, i, S, E, a
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
                "Ran Win::<T: {}, L: {}, I: {}, S: {}, E: {}, A: {}>::sort_events()",
                T, L, i, S, E, a
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
                "Ran set_poll::<T: {}, L: {}, I: {}, S: {}, E: {}, A: {}>(poll: {})",
                T, L, i, S, E, a, poll
            );
        }
        self.poll = poll;
    }
}

#[cfg(not(feature = "no_std"))]
pub mod easy {
    use super::*;

    pub type Button<const E: usize, A> = Wid<0, u8, 0, E, A>;
    pub type Label<const E: usize, A> = Wid<0, u8, 0, E, A>;
    pub type Window<const T: usize, const E: usize, A> = Win<T, 0, u8, 0, E, A>;

    impl<const E: usize, A> Button<E, A> {
        pub fn label_static<F>(mut self, f: F) -> Self
        where
            F: FnMut() -> String + 'static,
        {
            self.label(move |app| f());
            self
        }

        pub fn on_click<F>(mut self, f: F) -> Self
        where
            F: FnMut(&mut A) + 'static,
        {
            self.fun(move |app, _| {
                f(app);
            });
            self
        }
    }

    impl<const E: usize, A> Label<E, A> {
        pub fn label_dynamic<F>(mut self, f: F) -> Self
        where
            F: FnMut(&mut A) -> String + 'static,
        {
            self.label(move |app| f(app));
            self
        }
    }

    impl<const T: usize, const E: usize, A> Window<T, E, A> {
        pub fn title<F>(mut self, f: F) -> Self
        where
            F: FnMut(&mut A) -> String + 'static,
        {
            self.title(move |app| f(app));
            self
        }

        pub fn widgets(&mut self) -> &mut Vec<Wid<0, u8, 0, E, A>> {
            &mut self.widgets
        }
    }
}

pub use prelude::*;

pub mod prelude {
    pub use crate::easy::{Button, Label, Window};
    pub use crate::{Num, PinionsApp, Point, Rect};
}
