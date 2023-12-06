# 代码禁用开门狗

## 启用项目成员

> vim Cargo.toml

```toml
[workspace]
members = [
    "app/wdg/code_disable_wdg",
]
```

## 执行指令

```shell
cargo run -p code_disable_wdg
```
