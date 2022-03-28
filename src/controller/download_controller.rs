use crate::utils::download;
use indicatif::ProgressBar;
use std::error::Error;
use std::net::IpAddr;
use std::time::Duration;

pub async fn download_controller(
    ips: Vec<(IpAddr, Duration)>,
) -> Result<Vec<(IpAddr, Duration, usize)>, Box<dyn Error>> {
    println!("将对 {} 个 ip 进行下载速度测试", ips.len());
    let pb = ProgressBar::new(ips.len().try_into().unwrap());
    pb.tick();
    let mut result_vec = Vec::new();
    for ip in ips {
        let res = download(ip.0).await?;
        result_vec.push((ip.0, ip.1, res));
        pb.inc(1);
    }
    pb.finish();
    result_vec.sort_by(|b, a| a.2.cmp(&b.2));
    Ok(result_vec)
}
