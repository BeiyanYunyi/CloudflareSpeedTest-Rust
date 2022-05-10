# ⚡ Cloudflare Speed Test in Rust

[![GitHub stars](https://img.shields.io/github/stars/lixiang810/cloudflare-speed-test-rust?style=for-the-badge)](https://github.com/lixiang810/cloudflare-speed-test-rust/stargazers) [![GitHub license](https://img.shields.io/github/license/lixiang810/cloudflare-speed-test-rust?style=for-the-badge)](https://github.com/lixiang810/cloudflare-speed-test-rust/blob/main/LICENSE)

Cloudflare Speed Test written in Rust, for my practice.

## 🔖 Download Release

[Here](https://github.com/lixiang810/cloudflare-speed-test-rust/releases/)

### 📦 Choose file

| OS      | File to download                                                                                                                                   |
| ------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| Windows | `cloudflare-speed-test-rust_[Version]_x86_64-pc-windows-gnu.zip`                                                                                   |
| Linux   | `cloudflare-speed-test-rust_[Version]_x86_64-unknown-linux-musl.tar.xz` or `cloudflare-speed-test-rust_[Version]_x86_64-unknown-linux-musl.tar.gz` |
| Mac OS  | `cloudflare-speed-test-rust_[Version]_x86_64-apple-darwin.zip`                                                                                     |

## 🏭 Build it yourself

```bash
git clone https://github.com/lixiang810/cloudflare-speed-test-rust
cd cloudflare-speed-test-rust
cargo build -r
sudo ./target/release/cfst # sudo if you're using Linux
```

## 🔧 Use custom IP file

### Format

```plaintext
173.245.48.0/20
141.101.64.0/18
131.0.72.0/22
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

## ❤️ Thanks

- IBMYes（deleted）-- bash and bat
- better-cloudflare-ip（deleted）-- bash and bat
- [CloudflareSpeedTest](https://github.com/XIU2/CloudflareSpeedTest) -- Go

## 🔒 Privacy

This program will and will only communicate with the Cloudflare server with HTTPS and ICMP Protocol.

## 🤯 Disclaimer

Do anything with it at your own risk.

## ❓ Why not IPv6？

It is actually possible to support IPv6, with a few lines of code change. However, neither my house nor my school has IPv6, so I don't have the need for it.

## 🧑‍🏭 LICENSE

AGPL-3.0
