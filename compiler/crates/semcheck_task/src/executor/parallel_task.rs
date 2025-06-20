use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::PinnedTask;

pub fn join_all_task<I, T>(tasks: I) -> impl Future<Output = Result<Vec<T>, Vec<Box<dyn Error>>>>
where
    I: Iterator<Item = PinnedTask<T>>,
    T: Unpin,
{
    let pinned_tasks = tasks
        .map(Some)
        .collect::<Vec<_>>();
    let empty_artifacts = (0..pinned_tasks.len())
        .map(|_| None)
        .collect::<Vec<_>>();

    JoinAll {
        tasks: pinned_tasks,
        artifacts: empty_artifacts,
    }
}

struct JoinAll<T> {
    tasks: Vec<Option<PinnedTask<T>>>,
    artifacts: Vec<Option<T>>,
}

impl<T> Future for JoinAll<T>
where
    T: Unpin,
{
    type Output = Result<Vec<T>, Vec<Box<dyn Error>>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut all_completed = true;
        let mut any_stepped = false;

        // 管理下の全タスクを進める
        for idx in 0..self.tasks.len() {
            let mut task = self.tasks[idx].as_mut();
            if task.is_some() {
                match task.as_mut().unwrap().as_mut().poll(cx) {
                    Poll::Ready(result) => {
                        any_stepped = true;
                        self.tasks[idx] = None;
                        self.artifacts[idx] = Some(result);
                    }
                    Poll::Pending => {
                        all_completed = false;
                    }
                }
            }
        }

        // 全てのタスクが完了した場合
        if all_completed {
            let artifactis = self
                .artifacts
                .iter_mut()
                .map(|artifact| artifact.take().unwrap())
                .collect();
            return Poll::Ready(Ok(artifactis));
        }

        // 全てのタスクが完了していないが，いずれかのタスクが進行した場合
        if any_stepped {
            return Poll::Pending;
        }

        // いずれのタスクも進行しなかった場合
        let deadlock_errs = self
            .tasks
            .iter_mut()
            .filter(|task| task.is_some())
            .map(|task| task.as_mut().unwrap().on_deadlock())
            .collect::<Vec<_>>();

        Poll::Ready(Err(deadlock_errs))
    }
}

#[cfg(test)]
mod tests {
    use std::future::poll_fn;
    use std::task::Poll;

    use crate::{Task, block_on};
    use super::join_all_task;

    #[test]
    fn test_ok_1() {
        let task_a = Task::new(
            async { Ok::<i32, ()>(1) },
            Box::from("aaa"),
        );
        let tasks = [task_a].into_iter();

        assert_eq!(block_on(join_all_task(tasks)).unwrap(), vec![Ok(1)]);
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
        let tasks = [
            task_a,
            task_b,
            task_c,
        ].into_iter();

        assert_eq!(block_on(join_all_task(tasks)).unwrap(), vec![Ok(1), Ok(2), Ok(3)]);
    }

    #[test]
    fn test_err_1() {
        let task_a = Task::new(
            async { Ok::<i32, ()>(1) },
            Box::from("Task A failed"),
        );
        let task_b = Task::new(
            async { Ok::<i32, ()>(2) },
            Box::from("Task B failed"),
        );
        let task_never_complete = Task::new(
            poll_fn(|_| Poll::Pending),
            Box::from("Task C failed"),
        );
        let tasks = [
            task_a,
            task_b,
            task_never_complete,
        ].into_iter();

        assert!(block_on(join_all_task(tasks)).is_err());
    }
}
