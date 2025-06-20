use std::collections::VecDeque;

use sb_compiler_semcheck_task::{PinnedTask, Task, block_on, join_all_task};
use sb_compiler_semcheck_task_macros::task;

#[task(deadlock = "return_0")]
fn return_0(num: i32) -> Option<i32> {
    Some(num)
}

#[task(deadlock = "return_1")]
fn return_1(num: i32) -> Option<i32> {
    static mut CNT: i32 = 0;

    if unsafe { CNT } == 0 {
        unsafe { CNT += 1; }
        return None;
    } else {
        return Some(num);
    }
}

#[task(deadlock = format!("return_inf({})", _num))]
fn return_inf(_num: i32) -> Option<i32> {
    None
}

#[test]
fn test_ok() {
    let tasks = [
        return_0(1),
        return_1(2),
    ].into_iter();

    assert_eq!(block_on(join_all_task(tasks)).unwrap(), vec![1, 2]);
}

#[test]
fn test_err() {
    let tasks = [
        return_0(1),
        return_1(2),
        return_inf(3),
        return_inf(4),
    ].into_iter();

    let result = block_on(join_all_task(tasks));
    assert!(result.is_err());

    let mut result_err = VecDeque::from(result.err().unwrap());
    assert_eq!(result_err.pop_front().unwrap().to_string(), "return_inf(3)");
    assert_eq!(result_err.pop_front().unwrap().to_string(), "return_inf(4)");
}
