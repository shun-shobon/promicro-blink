# Pro Micro blink

> RustとPro MicroでLチカします

詳しくは[RustをPro Microで動かす](https://scrapbox.io/shunshobon/Rust%E3%82%92Pro_Micro%E3%81%A7%E5%8B%95%E3%81%8B%E3%81%99)を参照してください。

## 必要なもの

- Rustup
- AVR関連のツール
  - avr-gcc
  - avr-binutils
  - avr-libc
  - avrdude

## ビルド方法

1. クローンする
2. 下記のコマンド実行
  ```shell
  cargo build --release
  ```

## Pro Microへの書き込み方法

`avrdude`を使用して書き込む

```shell
avrdude -p atmega32u4 -c avr109 -P <port> -b 57600 -D -U flash:w:./target/avr-atmega32u4/release/promicro-blink.elf:e
```