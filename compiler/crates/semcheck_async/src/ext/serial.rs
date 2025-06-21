use std::future::Future;

use crate::block_on;

pub trait FutureExtSerial<T> {
    fn block_on(self) -> T;
}

impl<F, T> FutureExtSerial<T> for F
where
    F: Future<Output = T>,
{
    fn block_on(self) -> T {
        block_on(self)
    }
}
