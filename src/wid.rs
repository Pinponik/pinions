//!

use crate::*;

pub struct Wid<const L: usize, const S: usize = 0> {
    label: Str<L>,
    border: usize,
    border_color: Color,
    img_data: Vect<u8, S>,
    img_width: u32,
    img_height: u32,
}
