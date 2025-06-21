use std::future::Future;
use std::pin::Pin;

use sb_compiler_semcheck_async::{SharedState, block_on, join_all};
use sb_compiler_semcheck_async_macros::failable_as_async;

#[failable_as_async('a)]
fn return_0<'a>(num_s: &'a str) -> Option<i32> {
    num_s.parse().ok()
}

#[failable_as_async('a)]
fn return_1<'a>(num_s: &'a str) -> Option<i32> {
    static mut CNT: i32 = 0;

    if unsafe { CNT } == 0 {
        unsafe { CNT += 1; }
        return None;
    } else {
        return num_s.parse().ok();
    }
}

#[failable_as_async]
fn return_inf(_: &str) -> Option<i32> {
    None
}

#[test]
fn test_ok() {
    let tasks: [Pin<Box<dyn Future<Output = Option<i32>>>>; 2] = [
        Box::pin(return_0("1")),
        Box::pin(return_1("2")),
    ];

    assert_eq!(
        block_on(join_all(tasks.into_iter())).collect::<Vec<_>>(),
        vec![Some(1), Some(2)]
    );
}

#[test]
fn test_err() {
    let tasks: [Pin<Box<dyn Future<Output = Option<i32>>>>; 4] = [
        Box::pin(return_0("1")),
        Box::pin(return_1("2")),
        Box::pin(return_inf("3")),
        Box::pin(return_inf("4")),
    ];

    assert_eq!(
        block_on(join_all(tasks.into_iter())).collect::<Vec<_>>(),
        vec![Some(1), Some(2), None, None],
    );
}
