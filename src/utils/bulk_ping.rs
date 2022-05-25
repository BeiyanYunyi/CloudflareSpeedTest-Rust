use crate::utils::ping;
use futures::future::join_all;
use random_number::random;
use std::net::IpAddr::{V4, V6};
use surge_ping::{Client, Config, PingIdentifier, ICMP};

/// ## bulk_ping
/// 批量 Ping
pub async fn bulk_ping<'a>(
  ips: Vec<std::net::IpAddr>,
) -> Result<Vec<(std::net::IpAddr, std::time::Duration)>, Box<dyn std::error::Error>> {
  let client = match ips.get(0) {
    Some(V4(_)) => Client::new(&Config::default()).unwrap(),
    Some(V6(_)) => Client::new(&Config::builder().kind(ICMP::V6).build()).unwrap(),
    None => Client::new(&Config::default()).unwrap(),
  };
  let mut tasks = vec![];
  for ip in ips {
    let rand_num: u16 = random!();
    let pinger = client.pinger(ip, PingIdentifier(rand_num)).await;
    tasks.push(tokio::spawn(ping(ip, pinger)));
  }
  let task_results = join_all(tasks).await;
  let mut result = Vec::new();
  for task_result in task_results {
    let task_result_unwrapped = task_result.unwrap().unwrap();
    if task_result_unwrapped.1 != std::time::Duration::from_secs(2) {
      result.push(task_result_unwrapped);
    }
  }
  Ok(result)
}
