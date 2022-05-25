use std::net::IpAddr;
use std::time::Duration;
use surge_ping::{IcmpPacket, PingSequence, Pinger};
use tokio::time;

/// ## ping
/// Ping 一个 IP
pub async fn ping(
    ip: IpAddr,
    mut pinger: Pinger,
) -> Result<(IpAddr, Duration), Box<dyn std::error::Error + Send + Sync>> {
    const PAYLOAD: [u8; 64] = [255; 64];
    let timeout = Duration::from_secs(2);
    pinger.timeout(timeout);
    let mut avg_time = Duration::from_secs(0);
    let mut interval = time::interval(Duration::from_secs(1));
    for idx in 0..5 {
        interval.tick().await;
        match pinger.ping(PingSequence(idx), &PAYLOAD).await {
            Ok((IcmpPacket::V4(_packet), dur)) => {
                avg_time += dur;
            }
            Ok((IcmpPacket::V6(_packet), dur)) => {
                avg_time += dur;
            }
            Err(_) => {
                avg_time += timeout;
            }
        };
    }
    // println!("[+] {} done.", pinger.destination);
    Ok((ip, avg_time / 5))
}
