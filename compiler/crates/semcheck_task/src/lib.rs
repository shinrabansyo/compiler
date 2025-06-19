mod executor;
mod task;

pub use executor::serial::block_on;
pub use executor::parallel::join_all;
pub use task::{PinnedTask, Task, TaskAccessor};
