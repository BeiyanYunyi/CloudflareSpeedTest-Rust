use crate::utils::ping;
use futures::future::join_all;
use surge_ping::{Client, Config};

pub async fn bulk_ping(
    ips: Vec<std::net::Ipv4Addr>,
) -> Result<Vec<(std::net::Ipv4Addr, std::time::Duration)>, Box<dyn std::error::Error>> {
    let mut tasks = Vec::new();
    let client = Client::new(&Config::default()).await?;
    for ip in ips {
        tasks.push(tokio::spawn(ping(client.clone(), ip)))
    }
    let bbb = join_all(tasks).await;
    let mut result = Vec::new();
    for ccc in bbb {
        let ddd = ccc.unwrap().unwrap();
        if ddd.1 != std::time::Duration::from_secs(2) {
            result.push(ddd);
        }
    }
    Ok(result)
}
