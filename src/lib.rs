//! ```text
//!                        ____  _       _
//!                       / __ \(_)___  (_)___  ____  _____
//!                      / /_/ / / __ \/ / __ \/ __ \/ ___/
//!                     / ____/ / / / / / /_/ / / / (__  )
//!                    /_/   /_/_/ /_/_/\____/_/ /_/____/
//! ```
//! ```text
//!           A fast and easy-to-use GUI library running on microcontrollers
//! ```
//!
//! [![GitHub](https://img.shields.io/badge/github.com-Pinponik%2FPinions-000?logo=github)](https://github.com/Pinponik/Pinions/)
//! [![Docs](https://docs.rs/pinions/badge.svg)](https://docs.rs/pinions)
//! [![Crates.io](https://img.shields.io/crates/v/pinions)](https://crates.io/crates/pinions)
//! [![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](https://github.com/pinponik/pinions/blob/master/LICENSE)
//! [![Downloads](https://img.shields.io/crates/d/pinions.svg)](https://crates.io/crates/pinions)

#![cfg_attr(feature = "no_std", no_std)]

pub mod wid;
use wid::Wid;

pub mod win;

#[cfg(feature = "no_std")]
pub type Str<const N: usize> = heapless::String<N>;

#[cfg(not(feature = "no_std"))]
pub type Str<const N: usize> = String;

#[cfg(feature = "no_std")]
pub type Vect<T, const N: usize> = heapless::Vec<T, N>;

#[cfg(not(feature = "no_std"))]
pub type Vect<T, const N: usize = 0> = Vec<T>;

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct Mouse {
    pub x: f32,
    pub y: f32,
    pub clicked: bool,
}
