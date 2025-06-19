use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub type PinnedTask<T> = Pin<Box<dyn TaskAccessor<T> + Unpin>>;

pub trait TaskAccessor<T>
where
    Self: Future<Output = T>,
{
    fn on_deadlock(&mut self) -> Box<dyn Error>;
}

pub struct Task<F, T>
where
    F: Future<Output = T>,
{
    task: Pin<Box<F>>,
    on_deadlock_err: Option<Box<dyn Error>>,
}

impl<F, T> Future for Task<F, T>
where
    F: Future<Output = T>,
{
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.get_mut().task.as_mut().poll(cx)
    }
}

impl<F, T> TaskAccessor<T> for Task<F, T>
where
    F: Future<Output = T>,
{
    fn on_deadlock(&mut self) -> Box<dyn Error> {
        self.on_deadlock_err.take().unwrap()
    }
}

impl<F, T> Task<F, T>
where
    F: Future<Output = T> + 'static,
    T: 'static,
{
    pub fn new(task: F, on_deadlock_err: Box<dyn Error>) -> PinnedTask<T> {
        let task = Task {
            task: Box::pin(task),
            on_deadlock_err: Some(on_deadlock_err),
        };
        Box::pin(task)
    }
}
