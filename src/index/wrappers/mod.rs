// imports

mod click;

use crate::traits::Initialise;
use super::Index;

// exports

pub use click::ClickWrapper;


pub trait Wrapper: Index + Initialise {}