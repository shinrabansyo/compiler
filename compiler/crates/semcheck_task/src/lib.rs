#![feature(context_ext)]
#![feature(local_waker)]

mod executor;
mod state;

pub use executor::serial::block_on;
pub use executor::parallel::join_all;
pub use state::SharedState;
