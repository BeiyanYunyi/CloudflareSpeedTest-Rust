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
) -> Result<Vec<DownloadRes>, Box<dyn Error>> {
    let pb = ProgressBar::new(ips.len().try_into().unwrap());
    pb.tick();
    pb.println(format!("将对 {} 个 ip 进行下载速度测试", ips.len()));
    let mut result_vec = Vec::new();
    for ip in ips {
        pb.println(format!("正在测试: {}", ip.ip));
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
