# SB Compiler

## Usage

```
$ cargo run -- input.sb output.obj
```

- `input.sb` : プログラム(入力)
- `output.obj` : オブジェクトファイル(出力)

## Examples

- [examples/helloworld](examples/helloworld)
- [examples/fizzbuzz](examples/fizzbuzz)
- [examples/expr](examples/expr)

```
$ cargo run examples/helloworld/main.sb main.obj
```

## Test

- [Success Set](./compiler/tests/success)
- [Fail Set](./compiler/tests/fail)

```
$ cargo test -p sb_compiler
```
