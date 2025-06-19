use std::future::Future;
use std::task::{Context, Poll};

use futures::task::noop_waker_ref;
use futures::pin_mut;

pub fn block_on<F, T>(future: F) -> T
where
    F: Future<Output = T>,
{
    let mut cx = Context::from_waker(noop_waker_ref());
    pin_mut!(future);
    loop {
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(result) => return result,
            Poll::Pending => continue,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Task;
    use super::block_on;

    #[test]
    fn test_ok_1() {
        let task_a = Task::new(
            async { Ok::<i32, ()>(1) },
            Box::from("Task A failed"),
        );

        assert_eq!(block_on(task_a).unwrap(), 1);
    }

    #[test]
    fn test_ok_2() {
        let task_a = Task::new(
            async { Ok::<i32, ()>(1) },
            Box::from("Task A failed"),
        );
        let task_b = Task::new(
            async { Ok::<i32, ()>(2) },
            Box::from("Task B failed"),
        );
        let task_c = Task::new(
            async { Ok::<i32, ()>(3) },
            Box::from("Task C failed"),
        );

        let async_f  = async {
            let a = task_a.await?;
            let b = task_b.await?;
            let c = task_c.await?;
            Ok::<i32, ()>(a + b + c)
        };

        assert_eq!(block_on(async_f).unwrap(), 6);
    }
}
