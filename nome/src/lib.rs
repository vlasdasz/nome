#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]

mod app;
mod interface;

pub use app::NomeApp;
#[cfg(not(ios))]
pub use test_engine;

#[cfg(ios)]
test_engine::register_app!(NomeApp);
