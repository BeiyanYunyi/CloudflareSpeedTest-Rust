use crate::i18n::interface::I18nItems;
use crate::utils::download;
use crate::utils::RealDelayRes;
use indicatif::ProgressBar;
use std::error::Error;
use std::net::IpAddr;
use std::time::Duration;

/// 下载测速返回值
pub struct DownloadRes {
  pub ip: IpAddr,
  pub ping: Duration,
  pub real_delay: Duration,
  pub speed: f64,
}

/// ## download_controller
/// 下载测速
pub async fn download_controller(
  ips: Vec<RealDelayRes>,
  i18n: &I18nItems<'_>,
) -> Result<Vec<DownloadRes>, Box<dyn Error>> {
  let pb = ProgressBar::new(ips.len().try_into().unwrap());
  pb.tick();
  pb.println(format!(
    "{} {} {}",
    i18n.download_controller_i18n.total_before_num,
    ips.len(),
    i18n.download_controller_i18n.total_after_num,
  ));
  let mut result_vec = Vec::new();
  for ip in ips {
    pb.println(format!(
      "{}{}",
      i18n.download_controller_i18n.testing, ip.ip
    ));
    let res = download(ip.ip).await?;
    result_vec.push(DownloadRes {
      ip: ip.ip,
      ping: ip.ping,
      real_delay: ip.real_delay,
      speed: res,
    });
    pb.inc(1);
  }
  pb.finish();
  result_vec.sort_by(|b, a| a.speed.partial_cmp(&b.speed).unwrap());
  Ok(result_vec)
}
