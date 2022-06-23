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

## ❤️ Thanks

- IBMYes（deleted）-- bash and bat
- better-cloudflare-ip（deleted）-- bash and bat
- [CloudflareSpeedTest](https://github.com/XIU2/CloudflareSpeedTest) -- Go

## 🔒 Privacy

This program will and will only communicate with the Cloudflare server with HTTPS and ICMP Protocol.

## 🤯 Disclaimer

Do anything with it at your own risk.

## 📝 Special Notes

### For IPv4

There are two copies of IPv4 IPs built into the program, one of which will be consistent with [Cloudflare's IP list](https://www.cloudflare.com/ips-v4). The other one was sent to me by a user, and may be of higher quality than Cloudflare's official list, but its access and security are not yet clear, so use at your own risk.

### For IPv6

As with IPv4, the project supports getting available IPs from [Cloudflare's IP list](https://www.cloudflare.com/ips-v6), but the vast majority of IPs in this IPv6 list are not available.

I got a list from [CloudflareSpeedTest](https://github.com/XIU2/CloudflareSpeedTest) and hardcoded it into the program. This list is very available, but its access and security are not clear.

If you are concerned about IP address security, you can let the program get IPs from Cloudflare (recommended test rounds are set to 20 or more), and if you want to be more efficient, you can let the program use the built-in IP list.

## 🧑‍🏭 LICENSE

AGPL-3.0
