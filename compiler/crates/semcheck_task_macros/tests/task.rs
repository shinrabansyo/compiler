use std::future::Future;
use std::pin::Pin;

use sb_compiler_semcheck_task::{SharedState, block_on, join_all};
use sb_compiler_semcheck_task_macros::failable_as_async;

#[failable_as_async]
fn return_0(num: i32) -> Option<i32> {
    Some(num)
}

#[failable_as_async]
fn return_1(num: i32) -> Option<i32> {
    static mut CNT: i32 = 0;

    if unsafe { CNT } == 0 {
        unsafe { CNT += 1; }
        return None;
    } else {
        return Some(num);
    }
}

#[failable_as_async]
fn return_inf(_num: i32) -> Option<i32> {
    None
}

#[test]
fn test_ok() {
    let tasks: [Pin<Box<dyn Future<Output = Option<i32>>>>; 2] = [
        Box::pin(return_0(1)),
        Box::pin(return_1(2)),
    ];

    assert_eq!(
        block_on(join_all(tasks.into_iter())),
        vec![Some(1), Some(2)]
    );
}

#[test]
fn test_err() {
    let tasks: [Pin<Box<dyn Future<Output = Option<i32>>>>; 4] = [
        Box::pin(return_0(1)),
        Box::pin(return_1(2)),
        Box::pin(return_inf(3)),
        Box::pin(return_inf(4)),
    ];

    assert_eq!(
        block_on(join_all(tasks.into_iter())),
        vec![Some(1), Some(2), None, None],
    );
}
