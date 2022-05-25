use crate::i18n::interface::I18nItems;
use crate::utils::{get_args, IPv6Range};
use dialoguer::{Confirm, Input};
use indicatif::ProgressBar;
use ipnet::{Ipv4Net, Ipv6Net};
use iprange::IpRange;
use random_number::random;
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;

const IP_CHUNK: u64 = 4096;

/// ## get_all_ips_v4
/// 获取 IPv4 IP，并返回
pub async fn get_all_ips_v4(
    i18n: &I18nItems<'_>,
) -> Result<Vec<IpAddr>, Box<dyn std::error::Error>> {
    let args = get_args();
    let ip_range: IpRange<Ipv4Net> = match args.custom_ip_file {
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
    // Disable simplify to make custom ranks possible.
    // ip_range.simplify();
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
    let ip_range: IpRange<Ipv6Net> = match args.custom_ip_file {
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
            let internal_list = [
                "2606:4700:3000::/48",
                "2606:4700:3001::/48",
                "2606:4700:3002::/48",
                "2606:4700:3003::/48",
                "2606:4700:3004::/48",
                "2606:4700:3005::/48",
                "2606:4700:3006::/48",
                "2606:4700:3007::/48",
                "2606:4700:3008::/48",
                "2606:4700:3009::/48",
                "2606:4700:3010::/48",
                "2606:4700:3011::/48",
                "2606:4700:3012::/48",
                "2606:4700:3013::/48",
                "2606:4700:3014::/48",
                "2606:4700:3015::/48",
                "2606:4700:3016::/48",
                "2606:4700:3017::/48",
                "2606:4700:3018::/48",
                "2606:4700:3019::/48",
                "2606:4700:3020::/48",
                "2606:4700:3021::/48",
                "2606:4700:3022::/48",
                "2606:4700:3023::/48",
                "2606:4700:3024::/48",
                "2606:4700:3025::/48",
                "2606:4700:3026::/48",
                "2606:4700:3027::/48",
                "2606:4700:3028::/48",
                "2606:4700:3029::/48",
                "2606:4700:3030::/48",
                "2606:4700:3031::/48",
                "2606:4700:3032::/48",
                "2606:4700:3033::/48",
                "2606:4700:3034::/48",
                "2606:4700:3035::/48",
                "2606:4700:3036::/48",
                "2606:4700:3037::/48",
                "2606:4700:3038::/48",
                "2606:4700:3039::/48",
            ]
            .iter()
            .map(|s| s.parse().unwrap())
            .collect::<IpRange<Ipv6Net>>();
            let if_internal = Confirm::new()
                .with_prompt(i18n.ping_controller_i18n.internal_or_online)
                .interact()?;
            match if_internal {
                false => {
                    let client = reqwest::ClientBuilder::new()
                        .timeout(Duration::from_secs(5))
                        .build()?;
                    let cf_ips = client.get("https://www.cloudflare.com/ips-v6").send().await;
                    println!("{}", i18n.ping_controller_i18n.getting_ips_from_cloudflare);
                    let res = match cf_ips {
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
                            internal_list
                        }
                    };
                    res
                }
                true => internal_list,
            }
        }
    };
    // Disable simplify to make custom ranks possible.
    // ip_range.simplify();
    let ipv6_net_vec = ip_range
        .iter()
        .map(|ipv6_net| IPv6Range::new(ipv6_net))
        .collect::<Vec<IPv6Range>>();
    // 算法问题（数组查重），总轮数过高会产生过多的资源消耗
    const TOTAL_LENGTH: u64 = 256 * IP_CHUNK;
    let rand_num = Input::new()
        .with_prompt::<String>(format!(
            "{}{}{}{}{}",
            i18n.ping_controller_i18n.prompt_part1,
            TOTAL_LENGTH / IP_CHUNK,
            i18n.ping_controller_i18n.prompt_part2,
            IP_CHUNK,
            i18n.ping_controller_i18n.prompt_part3,
        ))
        .default(1)
        .validate_with(|input: &u64| -> Result<(), &str> {
            if *input * IP_CHUNK > TOTAL_LENGTH {
                return Err(i18n.ping_controller_i18n.invalid_input);
            }
            Ok(())
        })
        .interact_text()
        .expect(i18n.ping_controller_i18n.invalid_input);
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
