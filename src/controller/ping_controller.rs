use crate::utils::bulk_ping;
use dialoguer::Input;
use indicatif::ProgressBar;
use ipnet::Ipv4Net;
use iprange::IpRange;
use random_number::random;
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;

/// ## get_all_ips_v4
/// 获取 IPv4 IP，并返回
async fn get_all_ips_v4() -> Result<Vec<IpAddr>, Box<dyn std::error::Error>> {
    println!("正在从 Cloudflare 获取 IP 列表");
    let client = reqwest::ClientBuilder::new()
        .timeout(Duration::from_secs(5))
        .build()?;
    let cf_ips = client.get("https://www.cloudflare.com/ips-v4").send().await;
    let ip_range: IpRange<Ipv4Net> = match cf_ips {
        Ok(res) => {
            println!("从 Cloudflare 获取 IP 列表成功");
            let res = res.text().await?;
            let res: Vec<&str> = res.trim().split("\n").collect();
            res.iter().map(|s| s.parse().unwrap()).collect()
        }
        Err(_) => {
            println!("从 Cloudflare 获取 IP 列表失败，使用内置 IP 列表");
            [
                "173.245.48.0/20",
                "103.21.244.0/22",
                "103.22.200.0/22",
                "103.31.4.0/22",
                "141.101.64.0/18",
                "108.162.192.0/18",
                "190.93.240.0/20",
                "188.114.96.0/20",
                "197.234.240.0/22",
                "198.41.128.0/17",
                "162.158.0.0/15",
                "104.16.0.0/13",
                "104.24.0.0/14",
                "172.64.0.0/13",
                "131.0.72.0/22",
            ]
            .iter()
            .map(|s| s.parse().unwrap())
            .collect()
        }
    };
    let mut ips_vec_temp: Vec<Ipv4Addr> = ip_range
        .iter()
        .flat_map(|ipv4_net| ipv4_net.hosts())
        .collect();
    let mut ips_vec = Vec::new();
    const IP_CHUNK: usize = 4096;
    let rand_num = Input::new()
        .with_prompt::<String>(format!(
            "请输入测试轮数 (0 ≤ x ≤ {}) (每轮 {} 个，用时 10 秒，互不重复) ",
            ips_vec_temp.len() / IP_CHUNK,
            IP_CHUNK,
        ))
        .default(1)
        .validate_with(|input: &usize| -> Result<(), &str> {
            if *input * IP_CHUNK > ips_vec_temp.len() {
                return Err("输入不合法");
            }
            Ok(())
        })
        .interact_text()
        .expect("输入无效");
    for _ in 0..(rand_num * IP_CHUNK) {
        let len = ips_vec_temp.len();
        ips_vec.push(IpAddr::V4(ips_vec_temp.swap_remove(random!(0..len))))
    }
    return Ok(ips_vec);
}

/// ## ping_controller
/// Ping 测试
pub async fn ping_controller() -> Result<Vec<(IpAddr, Duration)>, Box<dyn std::error::Error>> {
    let mut ips = get_all_ips_v4().await?;
    let pb = ProgressBar::new(ips.len().try_into().unwrap());
    println!("将对 {} 个 ip 进行 ping 测试", ips.len());
    pb.tick();
    let mut result = Vec::new();
    const BULK_NUM: usize = 4096;
    loop {
        if ips.len() == 0 {
            pb.finish();
            break;
        }
        if ips.len() < BULK_NUM {
            let mut res = bulk_ping(ips).await?;
            result.append(&mut res);
            pb.finish();
            break;
        }
        let ips_slice = ips.split_off(ips.len() - BULK_NUM);
        let mut res = bulk_ping(ips_slice).await?;
        result.append(&mut res);
        pb.inc(BULK_NUM.try_into().unwrap());
    }
    result.sort_by(|a, b| a.1.cmp(&b.1));
    result.truncate(10);
    Ok(result)
}
