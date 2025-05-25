use std::fs;
use std::panic;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Expect {
    Ok,
    Err,
}

pub fn test_dir<T>(dir: &str, expect: Expect, test_fn: &T)
where
    T: Fn(&str) -> anyhow::Result<String> + panic::RefUnwindSafe,
{
    let mut testcases = fs::read_dir(dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| { !path.is_file() })
        .map(|dir_path| {
            let input = fs::read_to_string(dir_path.join("input.txt")).unwrap();
            let output = fs::read_to_string(dir_path.join("output.txt")).unwrap();
            (dir_path, input, output)
        })
        .collect::<Vec<_>>();
    testcases.sort_by(|a, b| a.0.cmp(&b.0));

    for (dir, input, output) in testcases {
        println!("Testing {:?} ... ", dir);

        let result = panic::catch_unwind(|| test_fn(&input).unwrap());
        match result {
            Ok(..) if expect == Expect::Err => {
                println!("Failed (expected Error, but got Ok)");
                panic!("");
            }
            Ok(actual) => if actual != output && expect == Expect::Ok {
                println!("Failed (expected Ok, but got different result)");
                println!("=== Actual === ");
                println!("{}", actual);
                println!("=== Expected === ");
                println!("{}", output);
                panic!("");
            }
            Err(e) if expect == Expect::Ok => {
                println!("expected Ok, but got Error.");
                panic!("{}", e.downcast_ref::<anyhow::Error>().unwrap());
            }
            _ => println!("Ok"),
        }
    }
}
