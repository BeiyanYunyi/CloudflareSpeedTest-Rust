use crate::data::{IPV4_IPS_ORIGINAL, IPV4_IPS_TESTED, IPV6_IPS};
use crate::i18n::interface::I18nItems;
use crate::utils::{get_args, IPv6Range};
use async_recursion::async_recursion;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use indicatif::ProgressBar;
use ipnet::{Ipv4Net, Ipv6Net};
use iprange::IpRange;
use random_number::random;
use std::collections::HashSet;
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
      (max / IP_CHUNK) + 1,
      i18n.ping_controller_i18n.prompt_part2,
      IP_CHUNK,
      i18n.ping_controller_i18n.prompt_part3,
    ))
    .default(1)
    .validate_with(|input: &u64| -> Result<(), &str> {
      if (max / IP_CHUNK) + 1 < *input {
        return Err(i18n.ping_controller_i18n.invalid_input);
      }
      Ok(())
    })
    .interact_text()
    .expect(i18n.ping_controller_i18n.invalid_input)
}

#[async_recursion]
async fn choose_ipv4_ips(i18n: &I18nItems<'_>) -> Result<String, Box<dyn std::error::Error>> {
  let items = vec![
    i18n.choose_ips_i18n.use_online_ips,
    i18n.choose_ips_i18n.use_original_ips,
    i18n.choose_ips_i18n.use_tested_ips,
  ];
  let selection = Select::with_theme(&ColorfulTheme::default())
    .items(&items)
    .default(0)
    .interact()
    .expect(i18n.ping_controller_i18n.invalid_input);
  let res = match selection {
    0 => {
      let client = reqwest::ClientBuilder::new()
        .timeout(Duration::from_secs(5))
        .build()?;
      let online_ips_res = client.get("https://www.cloudflare.com/ips-v4").send().await;
      if let Ok(res) = online_ips_res {
        let ips_str = res.text().await?;
        ips_str
      } else {
        println!(
          "{}",
          i18n.ping_controller_i18n.getting_ips_from_cloudflare_failed
        );
        choose_ipv4_ips(i18n).await?
      }
    }
    1 => IPV4_IPS_ORIGINAL.to_string(),
    2 => IPV4_IPS_TESTED.to_string(),
    _ => IPV4_IPS_ORIGINAL.to_string(),
  };
  Ok(res)
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
      std::fs::read_to_string(route).expect(i18n.ping_controller_i18n.reading_custom_file_error)
    }
    None => choose_ipv4_ips(i18n).await?,
  };
  let ip_range: IpRange<Ipv4Net> = txt
    .trim()
    .split("\n")
    .map(|s| {
      let mut st = String::from(s);
      if !st.contains("/") {
        st.push_str("/32");
      }
      st.parse().unwrap()
    })
    .collect();
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
    if len <= 0 {
      break;
    }
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
      std::fs::read_to_string(route).expect(i18n.ping_controller_i18n.reading_custom_file_error)
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
            i18n
              .ping_controller_i18n
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
  let ip_range: IpRange<Ipv6Net> = txt
    .trim()
    .split("\n")
    .map(|s| {
      let mut st = String::from(s);
      if !st.contains("/") {
        st.push_str("/128");
      }
      st.parse().unwrap()
    })
    .collect();
  // Disable simplify to make custom ranks possible.
  // ip_range.simplify();
  let mut ipv6_net_vec = ip_range
    .iter()
    .map(|ipv6_net| IPv6Range::new(ipv6_net))
    .collect::<Vec<IPv6Range>>();
  let mut total_length = 0u64;
  ip_range.iter().for_each(|ipv6_net| {
    total_length =
      total_length.saturating_add(2u64.saturating_pow(128u32 - (ipv6_net.prefix_len() as u32)));
  });
  let rand_num = input_num_of_ips(total_length, i18n);
  let mut rand_ips_hashset = HashSet::new();
  let pb = ProgressBar::new((rand_num * IP_CHUNK).try_into().unwrap());
  pb.tick();
  pb.println(i18n.ping_controller_i18n.generating_ips);
  let mut conflict_count = 0;
  while (rand_ips_hashset.len() as u64) < (rand_num * IP_CHUNK) {
    if conflict_count >= 1000 {
      break;
    }
    let len = ipv6_net_vec.len();
    let rand_ip = ipv6_net_vec[random!(0..len)].get_random_ip();
    if !rand_ips_hashset.contains(&rand_ip) {
      rand_ips_hashset.insert(rand_ip);
      pb.inc(1);
    } else {
      conflict_count += 1;
    }
  }
  pb.finish();
  return Ok(rand_ips_hashset.iter().map(|ele| *ele).collect());
}
