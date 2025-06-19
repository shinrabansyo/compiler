use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub type PinnedTask<T, Eto> = Pin<Box<dyn TaskAccessor<T, Eto> + Unpin>>;

pub trait TaskAccessor<T, Eto>
where
    Self: Future<Output = T>,
{
    fn on_timeout(&mut self) -> Eto;
}

pub struct Task<F, T, Eto>
where
    F: Future<Output = T>,
{
    task: Pin<Box<F>>,
    on_timeout_err: Option<Eto>,
}

impl<F, T, Eto> Future for Task<F, T, Eto>
where
    F: Future<Output = T>,
    Eto: Unpin,
{
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.get_mut().task.as_mut().poll(cx)
    }
}

impl<F, T, Eto> TaskAccessor<T, Eto> for Task<F, T, Eto>
where
    F: Future<Output = T>,
    Eto: Unpin,
{
    fn on_timeout(&mut self) -> Eto {
        self.on_timeout_err.take().unwrap()
    }
}

impl<F, T, Eto> Task<F, T, Eto>
where
    F: Future<Output = T> + 'static,
    T: 'static,
    Eto: Unpin + 'static,
{
    pub fn new(task: F, on_timeout_err: Eto) -> PinnedTask<T, Eto> {
        let task = Task {
            task: Box::pin(task),
            on_timeout_err: Some(on_timeout_err),
        };
        Box::pin(task)
    }
}
