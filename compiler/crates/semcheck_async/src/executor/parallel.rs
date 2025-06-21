use std::future::Future;
use std::iter::Map;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::vec::IntoIter;

type Pinned<T> = Pin<Box<T>>;
type ResultIter<T> = Map<IntoIter<Option<T>>, fn(Option<T>) -> T>;

pub fn join_all<'a, I, T>(futures: I) -> impl Future<Output = ResultIter<T>> + 'a
where
    I: Iterator<Item = Pinned<dyn Future<Output = T> + 'a>>,
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
        artifacts: Some(empty_artifacts),
    }
}

struct JoinAll<'a, T> {
    futures: Vec<Option<Pinned<dyn Future<Output = T> + 'a>>>,
    artifacts: Option<Vec<Option<T>>>,
}

impl<'a, T> Future for JoinAll<'a, T>
where
    T: Unpin + 'a,
{
    type Output = ResultIter<T>;

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut all_completed = true;

        // 管理下の全 Future を進める
        for idx in 0..self.futures.len() {
            let mut future = self.futures[idx].as_mut();
            if future.is_some() {
                match future.as_mut().unwrap().as_mut().poll(ctx) {
                    Poll::Ready(result) => {
                        self.futures[idx] = None;
                        self.artifacts.as_mut().unwrap()[idx] = Some(result);
                    }
                    Poll::Pending => {
                        all_completed = false;
                    }
                }
            }
        }

        // 全ての Future が完了した場合
        if all_completed {
           let artifacts = self
                .artifacts
                .take()
                .unwrap()
                .into_iter()
                .map((|mut opt| { opt.take().unwrap() }) as fn(Option<T>) -> T);
            return Poll::Ready(artifacts);
        }

        Poll::Pending
    }
}

#[cfg(test)]
mod tests {
    use std::future::{Future, poll_fn};
    use std::task::Poll;
    use std::pin::Pin;

    use crate::prelude::*;

    #[test]
    fn test_ok_1() {
        let tasks: [Pin<Box<dyn Future<Output = Result<i32, ()>>>>; 1] = [
            Box::pin(async { Ok(1) }),
        ];

        let result = tasks
            .into_iter()
            .join_all()
            .block_on()
            .collect::<Vec<_>>();

        assert_eq!(result, vec![Ok(1)]);
    }

    #[test]
    fn test_ok_2() {
        let tasks: [Pin<Box<dyn Future<Output = Result<i32, ()>>>>; 3] = [
            Box::pin(async { Ok(1) }),
            Box::pin(async { Ok(2) }),
            Box::pin(async { Ok(3) }),
        ];

        let result = tasks.into_iter()
            .join_all()
            .block_on()
            .collect::<Vec<_>>();

        assert_eq!(result, vec![Ok(1), Ok(2), Ok(3)]);
    }

    #[test]
    fn test_err_1() {
        let tasks: [Pin<Box<dyn Future<Output = Result<i32, ()>>>>; 3] = [
            Box::pin(async { Ok(1) }),
            Box::pin(async { Ok(2) }),
            Box::pin(poll_fn(|ctx| {
                if SharedState::check_deadlock(ctx) {
                    Poll::Ready(Err(()))
                } else {
                    Poll::Pending
                }
            })),
        ];

        let result = tasks.into_iter()
            .join_all()
            .block_on()
            .collect::<Vec<_>>();

        assert_eq!(result, vec![Ok(1), Ok(2), Err(())]);
    }
}
