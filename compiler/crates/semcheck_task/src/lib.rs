mod executor;
mod task;

pub use executor::serial::block_on;
pub use executor::parallel_std::join_all_std;
pub use executor::parallel_task::join_all_task;
pub use task::{PinnedTask, Task, TaskAccessor};
