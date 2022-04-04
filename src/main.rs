mod controller;
use dialoguer::Confirm;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ping_res = controller::ping_controller().await?;
    let real_delay_res = controller::real_delay_controller(ping_res).await?;
    let download_res = controller::download_controller(real_delay_res).await?;
    let mut cli_output_str = String::from("测试结果：\n");
    let mut file_output_str = String::from("IP,Ping,Real Delay,Speed (MB/s)\n");
    for res in download_res {
        let mb_s = res.speed / 1024f64 / 1024f64;
        let cli_line = format!(
            "IP: {:15}, Ping: {:4}, 真实延迟: {:4}, 速度: {:.5} MB/s\n",
            res.ip,
            res.ping.as_millis(),
            res.real_delay.as_millis(),
            mb_s,
        );
        let file_line = format!(
            "{},{},{},{:.5}\n",
            res.ip,
            res.ping.as_millis(),
            res.real_delay.as_millis(),
            mb_s
        );
        cli_output_str.push_str(cli_line.as_str());
        file_output_str.push_str(file_line.as_str());
    }
    print!("{}", cli_output_str);
    let if_save_file = Confirm::new().with_prompt("是否保存结果？").interact()?;
    if if_save_file {
        let mut div = std::env::current_dir().expect("无法获取程序运行目录");
        div.push("result.csv");
        std::fs::write(div, file_output_str).expect("文件写入失败");
        println!("结果已保存至运行目录的 result.csv 下");
        std::thread::sleep(std::time::Duration::from_secs(3));
    }
    Ok(())
}
