use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub type PinnedTask<'a, T> = Pin<Box<dyn TaskAccessor<'a, T> + Unpin>>;

pub trait TaskAccessor<'a, T>
where
    Self: Future<Output = T> + 'a,
{
    fn on_deadlock(&mut self) -> Box<dyn Error>;
}

pub struct Task<'a, F, T>
where
    F: Future<Output = T> + 'a,
{
    task: Pin<Box<F>>,
    on_deadlock_err: Option<Box<dyn Error>>,
    _phantom: &'a (),
}

impl<'a, F, T> Future for Task<'a, F, T>
where
    F: Future<Output = T> + 'a,
{
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.get_mut().task.as_mut().poll(cx)
    }
}

impl<'a, F, T> TaskAccessor<'a, T> for Task<'a, F, T>
where
    F: Future<Output = T> + 'a,
    T: 'a,
{
    fn on_deadlock(&mut self) -> Box<dyn Error> {
        self.on_deadlock_err.take().unwrap()
    }
}

impl<'a, F, T> Task<'a, F, T>
where
    F: Future<Output = T> + 'a,
    T: 'a,
{
    pub fn new(task: F, on_deadlock_err: Box<dyn Error>) -> PinnedTask<'a, T> {
        let task = Task {
            task: Box::pin(task),
            on_deadlock_err: Some(on_deadlock_err),
            _phantom: &(),
        };
        Box::pin(task)
    }
}
