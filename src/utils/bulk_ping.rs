use crate::utils::ping;
use futures::future::join_all;
use surge_ping::{Client, Config};

/// ## bulk_ping
/// 批量 Ping
pub async fn bulk_ping(
    ips: Vec<std::net::IpAddr>,
) -> Result<Vec<(std::net::IpAddr, std::time::Duration)>, Box<dyn std::error::Error>> {
    let mut tasks = Vec::new();
    let client = Client::new(&Config::default()).await?;
    for ip in ips {
        tasks.push(tokio::spawn(ping(client.clone(), ip)))
    }
    let task_results = join_all(tasks).await;
    let mut result = Vec::new();
    for task_result in task_results {
        let task_result_unwrapped = task_result.unwrap().unwrap();
        if task_result_unwrapped.1 != std::time::Duration::from_secs(2) {
            result.push(task_result_unwrapped);
        }
    }
    Ok(result)
}
