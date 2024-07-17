#![allow(incomplete_features)]
#![feature(const_trait_impl)]
#![feature(adt_const_params)]

extern crate core;

mod animation;
pub mod axis;
mod color;
pub mod converter;
pub mod flat;
mod misc;
mod num;
pub mod sign;
pub mod volume;

pub use animation::Animation;
pub use color::*;
pub use misc::{Apply, Platform, Toggle};
pub use num::{
    checked_convert::{checked_usize_to_u32, CheckedConvert},
    into_f32::ToF32,
    lossy_convert::LossyConvert,
    CheckedSub, IsZero, Min, MyAdd, One, Zero,
};
