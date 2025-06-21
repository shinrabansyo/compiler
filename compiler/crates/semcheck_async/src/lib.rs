#![feature(context_ext)]
#![feature(local_waker)]

mod executor;
mod ext;
mod state;

pub mod prelude {
    pub use crate::state::SharedState;
    pub use crate::ext::serial::FutureExtSerial;
    pub use crate::ext::parallel::FutureExtParallel;
}

pub use executor::serial::block_on;
pub use executor::parallel::join_all;
