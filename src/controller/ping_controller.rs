use crate::i18n::interface::I18nItems;
use crate::utils::bulk_ping;
use crate::utils::get_args;
use dialoguer::Input;
use indicatif::ProgressBar;
use ipnet::Ipv4Net;
use iprange::IpRange;
use random_number::random;
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;

/// ## get_all_ips_v4
/// 获取 IPv4 IP，并返回
async fn get_all_ips_v4(i18n: &I18nItems<'_>) -> Result<Vec<IpAddr>, Box<dyn std::error::Error>> {
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
    let mut ips_vec_temp: Vec<Ipv4Addr> = ip_range
        .iter()
        .flat_map(|ipv4_net| ipv4_net.hosts())
        .collect();
    let mut ips_vec = Vec::new();
    const IP_CHUNK: usize = 4096;
    let rand_num = Input::new()
        .with_prompt::<String>(format!(
            "{}{}{}{}{}",
            i18n.ping_controller_i18n.prompt_part1,
            ips_vec_temp.len() / IP_CHUNK,
            i18n.ping_controller_i18n.prompt_part2,
            IP_CHUNK,
            i18n.ping_controller_i18n.prompt_part3,
        ))
        .default(1)
        .validate_with(|input: &usize| -> Result<(), &str> {
            if *input * IP_CHUNK > ips_vec_temp.len() {
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

/// ## ping_controller
/// Ping 测试
pub async fn ping_controller(
    i18n: &I18nItems<'_>,
) -> Result<Vec<(IpAddr, Duration)>, Box<dyn std::error::Error>> {
    let mut ips = get_all_ips_v4(i18n).await?;
    let pb = ProgressBar::new(ips.len().try_into().unwrap());
    println!(
        "{} {} {}",
        i18n.ping_controller_i18n.will_test_before_num,
        ips.len(),
        i18n.ping_controller_i18n.will_test_after_num
    );
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
    Ok(result)
}
