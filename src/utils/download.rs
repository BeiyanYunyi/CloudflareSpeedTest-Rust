use reqwest::Url;
use std::net::{IpAddr, SocketAddr};

pub async fn download(ip: IpAddr) -> Result<usize, Box<dyn std::error::Error>> {
    let url_string = format!(
        "https://speed.cloudflare.com/__down?bytes={}",
        1024 * 1024 * 1024
    );
    let url_str = url_string.get(0..url_string.len()).unwrap();
    let url = Url::parse(url_str)?;
    let client_builder = reqwest::ClientBuilder::new()
        .resolve("speed.cloudflare.com", SocketAddr::new(ip, 443))
        .no_proxy();
    let client = client_builder.build().unwrap();
    let req = client.get(url).build().unwrap();
    let mut resp_raw = match client.execute(req).await {
        Ok(t) => t,
        Err(_) => return Ok(0usize),
    };
    let mut data_len: usize = 0;
    let now = std::time::Instant::now();
    while let Some(chunk) = resp_raw.chunk().await? {
        if now.elapsed().as_secs() <= 10 {
            data_len += chunk.len();
        } else {
            data_len += chunk.len();
            break;
        }
    }
    Ok(data_len / now.elapsed().as_secs() as usize)
}
