use sb_compiler_semcheck_task::{PinnedTask, Task, block_on, join_all};
use sb_compiler_semcheck_task_macros::task;

#[task(timeout: String = "return_0".to_string())]
fn return_0(num: i32) -> Option<i32> {
    Some(num)
}

#[task(timeout: String = "return_1".to_string())]
fn return_1(num: i32) -> Option<i32> {
    static mut CNT: i32 = 0;

    if unsafe { CNT } == 0 {
        unsafe { CNT += 1; }
        return None;
    } else {
        return Some(num);
    }
}

#[task(timeout: String = format!("return_inf({})", _num))]
fn return_inf(_num: i32) -> Option<i32> {
    None
}

#[test]
fn test_ok() {
    let tasks = [
        return_0(1),
        return_1(2),
    ].into_iter();

    assert_eq!(block_on(join_all(tasks)).unwrap(), vec![1, 2]);
}

#[test]
fn test_err() {
    let tasks = [
        return_0(1),
        return_1(2),
        return_inf(3),
        return_inf(4),
    ].into_iter();

    let result = block_on(join_all(tasks));
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), vec!["return_inf(3)", "return_inf(4)"]);
}
