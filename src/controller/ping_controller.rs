use crate::utils::{bulk_ping, pick_usize};
use indicatif::ProgressBar;
use ipnet::Ipv4Net;
use iprange::IpRange;
use random_number::random;
use std::net::Ipv4Addr;
use std::time::Duration;

fn get_all_ips_v4() -> Vec<Ipv4Addr> {
    let ip_range: IpRange<Ipv4Net> = [
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
    .collect();
    let mut ips_vec_temp = Vec::new();
    ip_range
        .iter()
        .flat_map(|ipv4_net| ipv4_net.hosts())
        .for_each(|bbb| {
            ips_vec_temp.push(bbb);
        });
    let mut ips_vec = Vec::new();
    let rand_num = pick_usize(
        0,
        ips_vec_temp.len(),
        ips_vec_temp.len() / 100,
        "请输入希望测试的 IP 数",
    );
    for _i in 0..rand_num {
        let len = ips_vec_temp.len();
        ips_vec.push(ips_vec_temp.swap_remove(random!(0..len)))
    }
    return ips_vec;
}

pub async fn ping_controller() -> Result<Vec<(Ipv4Addr, Duration)>, Box<dyn std::error::Error>> {
    let mut ips = get_all_ips_v4();
    let pb = ProgressBar::new(ips.len().try_into().unwrap());
    println!("将对 {} 个 ip 进行 ping 测试", ips.len());
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
    result.truncate(10);
    Ok(result)
}
