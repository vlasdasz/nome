#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]

mod app;
mod interface;
mod metronome;

pub use app::NomeApp;
#[cfg(not(ios))]
pub use hilen;

#[cfg(ios)]
hilen::register_app!(NomeApp);
