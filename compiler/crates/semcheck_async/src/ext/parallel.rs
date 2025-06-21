use std::future::Future;
use std::pin::Pin;

use crate::join_all;

pub trait FutureExtParallel<'a, T> {
    fn join_all(&mut self) -> impl Future<Output = impl Iterator<Item = T>> + 'a;
}

impl<'a, I, F, T> FutureExtParallel<'a, T> for I
where
    I: Iterator<Item = F>,
    F: Future<Output = T> + 'a,
    T: Unpin + 'a,
{
    fn join_all(&mut self) -> impl Future<Output = impl Iterator<Item = T>> + 'a {
        let pinned_futures = self.map(coerce_future);
        join_all(pinned_futures)
    }
}

fn coerce_future<'a, F, T>(f: F) -> Pin<Box<dyn Future<Output = T> + 'a>>
where
    F: Future<Output = T> + 'a,
{
    Box::pin(f)
}
