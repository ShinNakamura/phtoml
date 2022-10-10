# プレースホルダー埋め

- 第一引数は TOML
- 第二引数は プレースホルダー`{%ph%}`がしこまれてるテンプレートファイル
- プレースホルダーを TOML から読んだ値で埋めて、標準出力へ

```sh
cargo run -- tests/conf.toml tests/template.txt
```
