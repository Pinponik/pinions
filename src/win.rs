//!

use crate::*;

pub struct Win<const W: usize, const L: usize> {
    title: Str<L>,
    widgets: Vect<Wid<L>, W>,
    poll: bool,
}
