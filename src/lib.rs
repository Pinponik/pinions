//!                        ____  _       _
//!                       / __ \(_)___  (_)___  ____  _____
//!                      / /_/ / / __ \/ / __ \/ __ \/ ___/
//!                     / ____/ / / / / / /_/ / / / (__  )
//!                    /_/   /_/_/ /_/_/\____/_/ /_/____/
//!
//!
//!           A fast and easy-to-use GUI library running on microcontrolleres

#![cfg_attr(feature = "no_std", no_std)]

mod wid;

#[cfg(feature = "no_std")]
pub type Str<const N: usize> = heapless::String<N>;

#[cfg(not(feature = "no_std"))]
pub type Str<const N: usize> = String;
