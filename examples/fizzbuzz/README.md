# examples/fizzbuzz

## 実行

```sh
# main.sb をコンパイル
$ cargo run -- -o main.obj main.sb ../../library/dev/uart.sb

# アセンブリファイルを生成
$ sb-linker -o main.asm main.obj

# エミュレータで実行
$ sb-emulator-tui --inst=main.asm
```

## コンパイル結果の確認

```sh
$ sb-objdump main.obj
```
