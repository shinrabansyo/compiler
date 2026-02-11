# SB Compiler

## Usage

```
$ cargo run -o output.obj input_1.sb input_2.sb ...
```

- `input_(n).sb` : プログラム (入力)
- `output.obj` : オブジェクトファイル (出力)

## Examples

- [examples/helloworld](examples/helloworld) : HelloWorld 出力
- [examples/fizzbuzz](examples/fizzbuzz) : FizzBuzz 出力
- [examples/vec2](examples/vec2) : 2D ベクトル計算

## Library

- [library/alloc/alloc.sb](library/alloc/alloc.sb) : メモリ管理
- [library/dev/uart.sb](library/dev/uart.sb) : UART 出力

## Test

- [Success Set](./compiler/tests/success)
- [Fail Set](./compiler/tests/fail)

```
$ cargo test -p sb_compiler -- --nocapture
```
