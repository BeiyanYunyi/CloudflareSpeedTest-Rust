# Cloudflare Speed Test in Rust

[![GitHub stars](https://img.shields.io/github/stars/lixiang810/cloudflare-speed-test-rust?style=for-the-badge)](https://github.com/lixiang810/cloudflare-speed-test-rust/stargazers) [![GitHub license](https://img.shields.io/github/license/lixiang810/cloudflare-speed-test-rust?style=for-the-badge)](https://github.com/lixiang810/cloudflare-speed-test-rust/blob/main/LICENSE)

用 Rust 写的 Cloudflare Speed Test，练手用。

## 使用方法

```bash
git clone https://github.com/lixiang810/cloudflare-speed-test-rust
cd cloudflare-speed-test-rust
cargo build -r
sudo ./target/release/cfst # Linux 下需要 sudo，Windows 下直接双击运行即可
```

## 鸣谢项目 / 类似项目

- IBMYes（已删除）—— bash 和 bat
- better-cloudflare-ip（已删除）—— bash 和 bat
- [CloudflareSpeedTest](https://github.com/XIU2/CloudflareSpeedTest) —— Go

## 隐私说明

本项目会且只会与 Cloudflare 服务器进行 https 和 icmp 通信。

## 免责声明

想干嘛就干嘛。当然，后果自负。

## 开源协议

AGPL-3.0
