use std::fs;
use std::fmt::Debug;
use std::panic;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Expect {
    Ok,
    Err,
}

pub fn test_dir<T, U, E>(dir: &str, expect: Expect, test_fn: &T)
where
    T: Fn(&str) -> Result<U, E> + panic::RefUnwindSafe,
    E: Debug,
{
    let mut entries = fs::read_dir(dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| { path.is_file() })
        .map(|path| {
            let body = fs::read_to_string(&path).unwrap();
            (path, body)
        })
        .collect::<Vec<_>>();
    entries.sort();

    for (path, body) in entries {
        print!("Testing {:?} ... ", path);
        let result = panic::catch_unwind(|| test_fn(&body)).unwrap();
        match result {
            Ok(_) if expect == Expect::Err => {
                println!("Failed (expected Error, but got Ok)");
                panic!("");
            }
            Err(e) if expect == Expect::Ok => {
                println!("Failed (expected Ok, but got Error)\n{:?}", e);
                panic!("");
            }
            _ => println!("Ok"),
        }
    }
}
