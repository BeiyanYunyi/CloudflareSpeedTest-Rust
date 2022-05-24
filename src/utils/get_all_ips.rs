use crate::i18n::interface::I18nItems;
use crate::utils::get_args;
use dialoguer::Input;
use ipnet::{Ipv4Net, Ipv6Net};
use iprange::IpRange;
use random_number::random;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::time::Duration;

const IP_CHUNK: u64 = 4096;

/// ## get_all_ips_v4
/// 获取 IPv4 IP，并返回
pub async fn get_all_ips_v4(
    i18n: &I18nItems<'_>,
) -> Result<Vec<IpAddr>, Box<dyn std::error::Error>> {
    let args = get_args();
    let mut ip_range: IpRange<Ipv4Net> = match args.custom_ip_file {
        Some(route) => {
            println!(
                "{}: {}",
                i18n.ping_controller_i18n.reading_custom_file, route
            );
            let txt = std::fs::read_to_string(route)
                .expect(i18n.ping_controller_i18n.reading_custom_file_error);
            let txt: Vec<&str> = txt.trim().split("\n").collect();
            txt.iter().map(|s| s.parse().unwrap()).collect()
        }
        None => {
            let client = reqwest::ClientBuilder::new()
                .timeout(Duration::from_secs(5))
                .build()?;
            let cf_ips = client.get("https://www.cloudflare.com/ips-v4").send().await;
            println!("{}", i18n.ping_controller_i18n.getting_ips_from_cloudflare);
            match cf_ips {
                Ok(res) => {
                    println!(
                        "{}",
                        i18n.ping_controller_i18n
                            .getting_ips_from_cloudflare_success
                    );
                    let res = res.text().await?;
                    let res: Vec<&str> = res.trim().split("\n").collect();
                    res.iter().map(|s| s.parse().unwrap()).collect()
                }
                Err(_) => {
                    println!(
                        "{}",
                        i18n.ping_controller_i18n.getting_ips_from_cloudflare_failed
                    );
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
            }
        }
    };
    ip_range.simplify();
    let mut ips_vec_temp: Vec<Ipv4Addr> = ip_range
        .iter()
        .flat_map(|ipv4_net| ipv4_net.hosts())
        .collect();
    let mut ips_vec = Vec::new();
    let rand_num = Input::new()
        .with_prompt::<String>(format!(
            "{}{}{}{}{}",
            i18n.ping_controller_i18n.prompt_part1,
            (ips_vec_temp.len() as u64) / IP_CHUNK,
            i18n.ping_controller_i18n.prompt_part2,
            IP_CHUNK,
            i18n.ping_controller_i18n.prompt_part3,
        ))
        .default(1)
        .validate_with(|input: &u64| -> Result<(), &str> {
            if *input * IP_CHUNK > (ips_vec_temp.len() as u64) {
                return Err(i18n.ping_controller_i18n.invalid_input);
            }
            Ok(())
        })
        .interact_text()
        .expect(i18n.ping_controller_i18n.invalid_input);
    for _ in 0..(rand_num * IP_CHUNK) {
        let len = ips_vec_temp.len();
        ips_vec.push(IpAddr::V4(ips_vec_temp.swap_remove(random!(0..len))))
    }
    return Ok(ips_vec);
}

/// ## get_all_ips_v6
/// 获取 IPv6 IP，并返回
pub async fn get_all_ips_v6(
    i18n: &I18nItems<'_>,
) -> Result<Vec<IpAddr>, Box<dyn std::error::Error>> {
    let args = get_args();
    let mut ip_range: IpRange<Ipv6Net> = match args.custom_ip_file {
        Some(route) => {
            println!(
                "{}: {}",
                i18n.ping_controller_i18n.reading_custom_file, route
            );
            let txt = std::fs::read_to_string(route)
                .expect(i18n.ping_controller_i18n.reading_custom_file_error);
            let txt: Vec<&str> = txt.trim().split("\n").collect();
            txt.iter().map(|s| s.parse().unwrap()).collect()
        }
        None => {
            let client = reqwest::ClientBuilder::new()
                .timeout(Duration::from_secs(5))
                .build()?;
            let cf_ips = client.get("https://www.cloudflare.com/ips-v6").send().await;
            println!("{}", i18n.ping_controller_i18n.getting_ips_from_cloudflare);
            match cf_ips {
                Ok(res) => {
                    println!(
                        "{}",
                        i18n.ping_controller_i18n
                            .getting_ips_from_cloudflare_success
                    );
                    let res = res.text().await?;
                    let res: Vec<&str> = res.trim().split("\n").collect();
                    res.iter().map(|s| s.parse().unwrap()).collect()
                }
                Err(_) => {
                    println!(
                        "{}",
                        i18n.ping_controller_i18n.getting_ips_from_cloudflare_failed
                    );
                    [
                        "2400:cb00::/32",
                        "2606:4700::/32",
                        "2803:f800::/32",
                        "2405:b500::/32",
                        "2405:8100::/32",
                        "2a06:98c0::/29",
                        "2c0f:f248::/32",
                    ]
                    .iter()
                    .map(|s| s.parse().unwrap())
                    .collect()
                }
            }
        }
    };
    ip_range.simplify();
    let ipv6_net_vec = ip_range
        .iter()
        .flat_map(|ipv6_net| ipv6_net.subnets(48).unwrap())
        .collect::<Vec<Ipv6Net>>();
    let total_length = u64::MAX;
    let rand_num = Input::new()
        .with_prompt::<String>(format!(
            "{}{}{}{}{}",
            i18n.ping_controller_i18n.prompt_part1,
            total_length / IP_CHUNK,
            i18n.ping_controller_i18n.prompt_part2,
            IP_CHUNK,
            i18n.ping_controller_i18n.prompt_part3,
        ))
        .default(1)
        .validate_with(|input: &u64| -> Result<(), &str> {
            if *input * IP_CHUNK > total_length {
                return Err(i18n.ping_controller_i18n.invalid_input);
            }
            Ok(())
        })
        .interact_text()
        .expect(i18n.ping_controller_i18n.invalid_input);
    let mut rand_ips_vec = Vec::new();
    while (rand_ips_vec.len() as u64) < (rand_num * IP_CHUNK) {
        let mut octets = ipv6_net_vec[random!(0..ipv6_net_vec.len())]
            .network()
            .octets();
        for i in 7..16 {
            octets[i] = random!(0..255);
        }
        let new_ip = Ipv6Addr::new(
            ((octets[0] as u16) << 8) | octets[1] as u16,
            ((octets[2] as u16) << 8) | octets[3] as u16,
            ((octets[4] as u16) << 8) | octets[5] as u16,
            ((octets[6] as u16) << 8) | octets[7] as u16,
            ((octets[8] as u16) << 8) | octets[9] as u16,
            ((octets[10] as u16) << 8) | octets[11] as u16,
            ((octets[12] as u16) << 8) | octets[13] as u16,
            ((octets[14] as u16) << 8) | octets[15] as u16,
        );
        if !rand_ips_vec.contains(&new_ip) {
            rand_ips_vec.push(new_ip);
        }
    }
    return Ok(rand_ips_vec
        .iter()
        .map(|ipv6_addr| IpAddr::V6(*ipv6_addr))
        .collect());
}
