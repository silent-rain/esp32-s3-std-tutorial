# Ureq HTTP 客户端

这是一个使用 reqwest 库实现网络请求。

这是一个失败的示例。

```text
Error: http://neverssl.com/: Network Error: Protocol not available (os error 109)

Caused by:
    Protocol not available (os error 109)
```

注意： 不支持 HTTPS 协议。

## 执行指令

```shell
cargo run -r -p http_ureq_client
```
