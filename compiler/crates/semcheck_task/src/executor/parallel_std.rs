use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub fn join_all_std<'a, I, T>(futures: I) -> impl Future<Output = Vec<T>> + 'a
where
    I: Iterator<Item = Pin<Box<dyn Future<Output = T> + 'a>>>,
    T: Unpin + 'a,
{
    let pinned_futures = futures
        .map(Some)
        .collect::<Vec<_>>();
    let empty_artifacts = (0..pinned_futures.len())
        .map(|_| None)
        .collect::<Vec<_>>();

    JoinAll {
        futures: pinned_futures,
        artifacts: empty_artifacts,
    }
}

struct JoinAll<'a, T> {
    futures: Vec<Option<Pin<Box<dyn Future<Output = T> + 'a>>>>,
    artifacts: Vec<Option<T>>,
}

impl<'a, T> Future for JoinAll<'a, T>
where
    T: Unpin,
{
    type Output = Vec<T>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut all_completed = true;
        let mut any_stepped = false;

        // 管理下の全 Future を進める
        for idx in 0..self.futures.len() {
            let mut future = self.futures[idx].as_mut();
            if future.is_some() {
                match future.as_mut().unwrap().as_mut().poll(cx) {
                    Poll::Ready(result) => {
                        any_stepped = true;
                        self.futures[idx] = None;
                        self.artifacts[idx] = Some(result);
                    }
                    Poll::Pending => {
                        all_completed = false;
                    }
                }
            }
        }

        // 全ての Future が完了した場合
        if all_completed {
            let artifactis = self
                .artifacts
                .iter_mut()
                .map(|artifact| artifact.take().unwrap())
                .collect();
            return Poll::Ready(artifactis);
        }

        // 全てのタスクが完了していないが，いずれかのタスクが進行した場合
        if any_stepped {
            return Poll::Pending;
        }

        // いずれのタスクも進行しなかった場合
        panic!("Deadlock detected: no task made progress");
    }
}

#[cfg(test)]
mod tests {
    use std::future::{Future, poll_fn};
    use std::task::Poll;
    use std::pin::Pin;

    use crate::block_on;
    use super::join_all_std;

    #[test]
    fn test_ok_1() {
        let tasks: [Pin<Box<dyn Future<Output = Result<i32, ()>>>>; 1] = [
            Box::pin(async { Ok(1) }),
        ];
        let tasks = tasks.into_iter();

        assert_eq!(block_on(join_all_std(tasks)), vec![Ok(1)]);
    }

    #[test]
    fn test_ok_2() {
        let tasks: [Pin<Box<dyn Future<Output = Result<i32, ()>>>>; 3] = [
            Box::pin(async { Ok(1) }),
            Box::pin(async { Ok(2) }),
            Box::pin(async { Ok(3) }),
        ];
        let tasks = tasks.into_iter();

        assert_eq!(block_on(join_all_std(tasks)), vec![Ok(1), Ok(2), Ok(3)]);
    }

    #[test]
    #[should_panic]
    fn test_err_1() {
        let tasks: [Pin<Box<dyn Future<Output = Result<i32, ()>>>>; 3] = [
            Box::pin(async { Ok(1) }),
            Box::pin(async { Ok(2) }),
            Box::pin(poll_fn(|_| Poll::Pending)),
        ];
        let tasks = tasks.into_iter();

        block_on(join_all_std(tasks));
    }
}
