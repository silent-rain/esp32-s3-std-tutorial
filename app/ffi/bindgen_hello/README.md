# Bindgen Hello

这是一个使用 Rust 工具自动绑定 C 语言的案例。

## 安装 ARM GCC 编译环境

```shell
sudo pacman -S arm-none-eabi-gcc arm-none-eabi-newlib
```

## 编译

绑定是动态生成。

```shell
cargo build --target thumbv7m-none-eabi --package bindgen_hello
```

## lib 测试

```shell
cargo test --target thumbv7m-none-eabi -p bindgen_hello
```

## 指定测试

```shell
cargo test --target thumbv7m-none-eabi --package bindgen_hello --test hello_demo
```
