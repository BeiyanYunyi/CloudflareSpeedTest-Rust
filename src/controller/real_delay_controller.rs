use crate::utils::bulk_real_delay;
use crate::utils::RealDelayRes;
use indicatif::ProgressBar;
use std::error::Error;
use std::net::IpAddr;
use std::time::Duration;

pub async fn real_delay_controller(
    ips: Vec<(IpAddr, Duration)>,
) -> Result<Vec<RealDelayRes>, Box<dyn Error>> {
    let pb = ProgressBar::new(10);
    pb.tick();
    pb.println("将进行真实延迟测试以获得 10 个可用 ip ");
    let mut res_vec = Vec::new();
    let mut i_temp = 0;
    while res_vec.len() < 10 {
        let mut bulk_vec = Vec::new();
        for i in i_temp..i_temp + 10 {
            let item = ips[i];
            bulk_vec.push(item);
        }
        let res = bulk_real_delay(bulk_vec).await?;
        res.iter().for_each(|item| {
            pb.inc(1);
            res_vec.push(*item);
        });
        i_temp += 10;
    }
    res_vec.sort_by(|a, b| a.real_delay.cmp(&b.real_delay));
    res_vec.truncate(10);
    pb.finish();
    Ok(res_vec)
}
