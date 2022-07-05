# ⚡ Cloudflare Speed Test in Rust

[English](README-en_US.md)

![banner](https://socialify.git.ci/lixiang810/CloudflareSpeedTest-Rust/image?description=1&font=KoHo&forks=1&issues=1&language=1&name=1&owner=1&pattern=Circuit%20Board&pulls=1&stargazers=1&theme=Dark)

[![GitHub stars](https://img.shields.io/github/stars/lixiang810/cloudflare-speed-test-rust?style=for-the-badge)](https://github.com/lixiang810/cloudflare-speed-test-rust/stargazers) [![GitHub license](https://img.shields.io/github/license/lixiang810/cloudflare-speed-test-rust?style=for-the-badge)](https://github.com/lixiang810/cloudflare-speed-test-rust/blob/main/LICENSE)

用 Rust 写的 Cloudflare Speed Test，练手用。

## 🔖 下载发行版

前往[此处](https://github.com/lixiang810/cloudflare-speed-test-rust/releases/)下载。

### 📦 文件选择

| 操作系统 | 文件选择                                                                                                                                         |
| -------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| Windows  | `cloudflare-speed-test-rust_[版本号]_x86_64-pc-windows-gnu.zip`                                                                                  |
| Linux    | `cloudflare-speed-test-rust_[版本号]_x86_64-unknown-linux-musl.tar.xz` 或 `cloudflare-speed-test-rust_[版本号]_x86_64-unknown-linux-musl.tar.gz` |
| Mac OS   | `cloudflare-speed-test-rust_[版本号]_x86_64-apple-darwin.zip`                                                                                    |

### ⚡ 下载加速

可以参考 [GhProxy](https://ghproxy.com)

## 🏭 自构建方法

```bash
git clone https://github.com/lixiang810/cloudflare-speed-test-rust
cd cloudflare-speed-test-rust
cargo build -r
sudo ./target/release/cfst # Linux 下需要 sudo，Windows 下直接双击运行即可
```

## 🔧 使用自定义 IP 文件

### 文件格式

#### IPv4

```plaintext
173.245.48.0/20
141.101.64.0/18
131.0.72.0/22
...
```

#### IPv6

```plaintext
2606:4700:3000::/48
2606:4700:3001::/48
2606:4700:3002::/48
2606:4700:3003::/48
2606:4700:3004::/48
...
```

### POSIX

```bash
sudo cfst -c <FILE>
```

### Windows

```dos
cfst.exe -c <FILE>
```

## ❤️ 鸣谢项目 / 类似项目

- IBMYes（已删除）—— bash 和 bat
- better-cloudflare-ip（已删除）—— bash 和 bat
- [CloudflareSpeedTest](https://github.com/XIU2/CloudflareSpeedTest) —— Go

## 🔒 隐私说明

本项目会且只会与 Cloudflare 服务器进行 https 和 icmp 通信。

## 🤯 免责声明

想干嘛就干嘛。当然，后果自负。

## 📝 特殊说明

### IPv4 的内置 IP

程序中内置了两份 IPv4 IP，其中一份会与 [Cloudflare 的 IP 列表](https://www.cloudflare.com/ips-v4)保持一致。另一份则是由一位用户发给我的，质量可能比 Cloudflare 官方的列表更高，但其获取方式与安全性都尚不明确，使用后果自负。

### IPv6 的内置 IP

与 IPv4 时一样，本项目支持从 [Cloudflare 的 IP 列表](https://www.cloudflare.com/ips-v6)获取可用 IP，但这份 IPv6 列表中绝大部分 IP 是不可用的。

我从 [CloudflareSpeedTest](https://github.com/XIU2/CloudflareSpeedTest) 获取了一份列表并硬编码到了程序中。这份列表的可用度很高，但其获取方式与安全性都尚不明确。

介意 IP 地址安全性的可以让程序从 Cloudflare 获取 IP（推荐测试轮数设为 20 轮以上），希望效率更高的可以让程序使用内置的 IP 列表。

## 🧑‍🏭 开源协议

AGPL-3.0
