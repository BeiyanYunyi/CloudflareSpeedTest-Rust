use clap::Parser;

/// 运行参数
#[derive(Parser, Debug)]
#[clap(name = "Cloudflare Speed Test")]
#[clap(
    about = "---------------------\nCloudflare Speed Test\n---------------------\n\nTest ping, real delay and download speed of Cloudflare CDN Nodes to get the fastest IP (IPv4), without accessing any third-party servers."
)]
#[clap(version, author = "lixiang810")]
pub struct Args {
    /// Custom IP file route
    #[clap(long, short)]
    pub custom_ip_file: Option<String>,
}

pub fn get_args() -> Args {
    return Args::parse();
}
