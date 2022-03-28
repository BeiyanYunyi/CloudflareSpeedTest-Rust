mod controller;
mod utils;
use std::net::IpAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ping_res = controller::ping_controller().await?;
    let mut ips_to_download = Vec::new();
    for a_ping_res in ping_res {
        ips_to_download.push((IpAddr::V4(a_ping_res.0), a_ping_res.1));
    }
    let download_res = controller::download_controller(ips_to_download).await?;
    let mut cli_output_str = String::from("测试结果：\n");
    let mut file_output_str = String::from("IP,Ping,Speed (MiB/s)\n");
    for res in download_res {
        let mb_s: f32 = (res.2 as f32) / 1024f32 / 1024f32;
        let cli_line = format!(
            "IP: {:15}, Ping: {:4}, 速度 (MiB/s): {:5}\n",
            res.0,
            res.1.as_millis(),
            mb_s,
        );
        let file_line = format!("{:15},{:4},{:5}\n", res.0, res.1.as_millis(), mb_s);
        cli_output_str.push_str(cli_line.as_str());
        file_output_str.push_str(file_line.as_str());
    }
    print!("{}", cli_output_str);
    let mut div = std::env::current_dir().expect("无法获取程序运行目录");
    div.push("result.csv");
    std::fs::write(div, file_output_str).expect("文件写入失败");
    println!("结果已保存至运行目录的 result.csv 下");
    Ok(())
}
