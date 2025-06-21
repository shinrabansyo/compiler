use std::future::Future;

use crate::block_on;

pub trait FutureExt<T> {
    fn block_on(self) -> T;
}

impl<F, T> FutureExt<T> for F
where
    F: Future<Output = T>,
{
    fn block_on(self) -> T {
        block_on(self)
    }
}
