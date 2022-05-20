use crate::i18n::interface::I18nItems;
use crate::utils::bulk_ping;
use crate::utils::{get_all_ips_v4, get_all_ips_v6};
use dialoguer::{theme::ColorfulTheme, Select};
use indicatif::ProgressBar;
use std::net::IpAddr;
use std::time::Duration;

/// ## ping_controller
/// Ping 测试
pub async fn ping_controller(
    i18n: &I18nItems<'_>,
) -> Result<Vec<(IpAddr, Duration)>, Box<dyn std::error::Error>> {
    let items = vec!["IPv4", "IPv6"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact()?;
    let mut ips = match selection {
        1 => get_all_ips_v6(i18n).await?,
        _ => get_all_ips_v4(i18n).await?,
    };
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
