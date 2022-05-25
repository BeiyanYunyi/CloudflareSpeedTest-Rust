use futures::future::join_all;
use reqwest::ClientBuilder;
use std::net::{IpAddr, SocketAddr};
use std::time::{Duration, Instant};

/// 真实延迟测试返回值
#[derive(Copy, Clone)]
pub struct RealDelayRes {
  pub ip: IpAddr,
  pub ping: Duration,
  pub real_delay: Duration,
}

/// ## bulk_real_delay
/// 批量测试真实延迟
pub async fn bulk_real_delay(
  ips: Vec<(IpAddr, Duration)>,
) -> Result<Vec<RealDelayRes>, Box<dyn std::error::Error>> {
  let mut tasks = Vec::new();
  for ip in ips {
    tasks.push(tokio::spawn(real_delay(ip)));
  }
  let task_results = join_all(tasks).await;
  let mut result = Vec::new();
  for task_result in task_results {
    let task_result_unwrapped = task_result.unwrap();
    match task_result_unwrapped {
      Ok(res) => result.push(RealDelayRes {
        ip: res.0,
        ping: res.1,
        real_delay: res.2,
      }),
      Err(_) => {}
    }
  }
  Ok(result)
}

/// ## real_delay
/// 测试真实延迟
async fn real_delay(
  ip: (IpAddr, Duration),
) -> Result<(IpAddr, Duration, Duration), Box<dyn std::error::Error + Sync + Send>> {
  let client = ClientBuilder::new()
    .resolve("www.cloudflare.com", SocketAddr::new(ip.0, 443))
    .no_proxy()
    .timeout(Duration::from_secs(2))
    .build()
    .unwrap();
  let dur = Instant::now();
  client.get("https://www.cloudflare.com").send().await?;
  return Ok((ip.0, ip.1, dur.elapsed()));
}
