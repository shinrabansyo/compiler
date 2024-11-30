# SB Compiler

## Examples

- [examples/helloworld](examples/helloworld)
- [examples/expr](examples/expr)

```
$ cargo run examples/helloworld/main.sb out.asm
```

## Test

- [Success Set](./compiler/tests/success)
- [Fail Set](./compiler/tests/fail)

```
$ cargo test -p sb_compiler
```
