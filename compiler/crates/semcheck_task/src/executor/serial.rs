use std::future::Future;
use std::sync::Arc;
use std::task::{ContextBuilder, Poll};

use futures::task::noop_waker_ref;
use futures::pin_mut;

use crate::state::SharedStateBuilder;

pub fn block_on<F, T>(future: F) -> T
where
    F: Future<Output = T>,
{
    pin_mut!(future);

    let mut deadlock_happened = false;
    loop {
        // async-tree 全体に通知する状態を準備
        let shared_state = SharedStateBuilder::new()
            .set_notify_deadlock(deadlock_happened)
            .build();

        // async 実行用の Context を準備
        let mut shared_state_for_ctx = Arc::clone(&shared_state);
        let mut ctx = ContextBuilder::from_waker(noop_waker_ref())
            .ext(&mut shared_state_for_ctx)
            .build();

        // 1 回だけ poll して結果を確認
        match future.as_mut().poll(&mut ctx) {
            Poll::Ready(result) => return result,
            Poll::Pending => {
                deadlock_happened = !shared_state.lock().unwrap().check_any_stepped();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::future::poll_fn;
    use std::task::{Context, Poll};

    use crate::SharedState;
    use super::block_on;

    #[test]
    fn test_run_ok_1() {
        let task_a = async { Ok::<i32, ()>(1) };

        assert_eq!(block_on(task_a).unwrap(), 1);
    }

    #[test]
    fn test_run_ok_2() {
        let task_a = async { Ok::<i32, ()>(1) };
        let task_b = async { Ok::<i32, ()>(2) };
        let task_c = async { Ok::<i32, ()>(3) };

        let async_f  = async {
            let a = task_a.await?;
            let b = task_b.await?;
            let c = task_c.await?;
            Ok::<i32, ()>(a + b + c)
        };

        assert_eq!(block_on(async_f).unwrap(), 6);
    }

    #[test]
    fn test_rw_state() {
        let task_a = poll_fn(|ctx: &mut Context| {
            let now_deadlock = SharedState::check_deadlock(ctx);
            Poll::Ready(now_deadlock)
        });

        assert_eq!(block_on(task_a), false);
    }
}
