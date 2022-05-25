use crate::data::{IPV4_IPS, IPV6_IPS};
use crate::i18n::interface::I18nItems;
use crate::utils::{get_args, IPv6Range};
use dialoguer::{Confirm, Input};
use indicatif::ProgressBar;
use ipnet::{Ipv4Net, Ipv6Net};
use iprange::IpRange;
use random_number::random;
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;

/// 每轮的 IP 数量
const IP_CHUNK: u64 = 4096;

/// ## input_num_of_ips
/// 让用户输入想测试的轮数
fn input_num_of_ips(max: u64, i18n: &I18nItems<'_>) -> u64 {
    Input::new()
        .with_prompt::<String>(format!(
            "{}{}{}{}{}",
            i18n.ping_controller_i18n.prompt_part1,
            max / IP_CHUNK,
            i18n.ping_controller_i18n.prompt_part2,
            IP_CHUNK,
            i18n.ping_controller_i18n.prompt_part3,
        ))
        .default(1)
        .validate_with(|input: &u64| -> Result<(), &str> {
            if *input * IP_CHUNK > max {
                return Err(i18n.ping_controller_i18n.invalid_input);
            }
            Ok(())
        })
        .interact_text()
        .expect(i18n.ping_controller_i18n.invalid_input)
}

/// ## get_all_ips_v4
/// 获取 IPv4 IP，并返回
pub async fn get_all_ips_v4(
    i18n: &I18nItems<'_>,
) -> Result<Vec<IpAddr>, Box<dyn std::error::Error>> {
    let args = get_args();
    let txt = match args.custom_ip_file {
        Some(route) => {
            println!(
                "{}: {}",
                i18n.ping_controller_i18n.reading_custom_file, route
            );
            std::fs::read_to_string(route)
                .expect(i18n.ping_controller_i18n.reading_custom_file_error)
        }
        None => {
            let client = reqwest::ClientBuilder::new()
                .timeout(Duration::from_secs(5))
                .build()?;
            let cf_ips = client.get("https://www.cloudflare.com/ips-v4").send().await;
            println!("{}", i18n.ping_controller_i18n.getting_ips_from_cloudflare);
            if let Ok(res) = cf_ips {
                println!(
                    "{}",
                    i18n.ping_controller_i18n
                        .getting_ips_from_cloudflare_success
                );
                res.text().await?
            } else {
                IPV4_IPS.to_string()
            }
        }
    };
    let ip_range: IpRange<Ipv4Net> = txt.trim().split("\n").map(|s| s.parse().unwrap()).collect();
    // Disable simplify to make custom ranks possible.
    // ip_range.simplify();
    let mut ips_vec_temp: Vec<Ipv4Addr> = ip_range
        .iter()
        .flat_map(|ipv4_net| ipv4_net.hosts())
        .collect();
    let mut ips_vec = Vec::new();
    let rand_num = input_num_of_ips(ips_vec_temp.len() as u64, i18n);
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
    let txt = match args.custom_ip_file {
        Some(route) => {
            println!(
                "{}: {}",
                i18n.ping_controller_i18n.reading_custom_file, route
            );
            std::fs::read_to_string(route)
                .expect(i18n.ping_controller_i18n.reading_custom_file_error)
        }
        None => {
            let if_internal = Confirm::new()
                .with_prompt(i18n.ping_controller_i18n.internal_or_online)
                .interact()?;
            if if_internal {
                IPV6_IPS.to_string()
            } else {
                let client = reqwest::ClientBuilder::new()
                    .timeout(Duration::from_secs(5))
                    .build()?;
                let cf_ips = client.get("https://www.cloudflare.com/ips-v6").send().await;
                println!("{}", i18n.ping_controller_i18n.getting_ips_from_cloudflare);
                let res = if let Ok(res) = cf_ips {
                    println!(
                        "{}",
                        i18n.ping_controller_i18n
                            .getting_ips_from_cloudflare_success
                    );
                    res.text().await?
                } else {
                    println!(
                        "{}",
                        i18n.ping_controller_i18n.getting_ips_from_cloudflare_failed
                    );
                    IPV6_IPS.to_string()
                };
                res
            }
        }
    };
    let ip_range: IpRange<Ipv6Net> = txt.trim().split("\n").map(|s| s.parse().unwrap()).collect();
    // Disable simplify to make custom ranks possible.
    // ip_range.simplify();
    let ipv6_net_vec = ip_range
        .iter()
        .map(|ipv6_net| IPv6Range::new(ipv6_net))
        .collect::<Vec<IPv6Range>>();
    // 算法问题（数组查重，复杂度O(N^2)），总轮数过高会产生过多的资源消耗
    const TOTAL_LENGTH: u64 = 256 * IP_CHUNK;
    let rand_num = input_num_of_ips(TOTAL_LENGTH, i18n);
    let mut rand_ips_vec = Vec::new();
    let pb = ProgressBar::new((rand_num * IP_CHUNK).try_into().unwrap());
    pb.tick();
    pb.println(i18n.ping_controller_i18n.generating_ips);
    while (rand_ips_vec.len() as u64) < (rand_num * IP_CHUNK) {
        let rand_ip = ipv6_net_vec[random!(0..ipv6_net_vec.len())].get_random_ip();
        if !rand_ips_vec.contains(&rand_ip) {
            rand_ips_vec.push(rand_ip);
            pb.inc(1);
        }
    }
    pb.finish();
    return Ok(rand_ips_vec);
}
