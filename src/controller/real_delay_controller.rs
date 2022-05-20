use crate::i18n::interface::I18nItems;
use crate::utils::bulk_real_delay;
use crate::utils::RealDelayRes;
use indicatif::ProgressBar;
use std::error::Error;
use std::net::IpAddr;
use std::time::Duration;

pub async fn real_delay_controller(
    ips: Vec<(IpAddr, Duration)>,
    i18n: &I18nItems<'_>,
) -> Result<Vec<RealDelayRes>, Box<dyn Error>> {
    let pb = ProgressBar::new(10);
    pb.tick();
    pb.println(format!("{}", i18n.real_delay_controller_i18n));
    let mut res_vec = Vec::new();
    let mut i_temp = 0;
    while res_vec.len() < 10 && res_vec.len() != ips.len() {
        let mut bulk_vec = Vec::new();
        for i in i_temp..i_temp + 10 {
            if i < ips.len() {
                bulk_vec.push(ips[i]);
            }
        }
        let res = bulk_real_delay(bulk_vec).await?;
        res.iter().for_each(|item| {
            if res_vec.len() < 10 {
                pb.inc(1);
                res_vec.push(*item);
            }
        });
        i_temp += 10;
    }
    res_vec.sort_by(|a, b| a.real_delay.cmp(&b.real_delay));
    pb.finish();
    Ok(res_vec)
}
