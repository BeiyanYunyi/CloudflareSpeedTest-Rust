mod controller;
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
mod i18n;
mod utils;
use i18n::{en_us::en_us, zh_cn::zh_cn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let items = vec!["简体中文", "English (US)"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact()?;
    let i18n_items = match selection {
        0 => zh_cn(),
        1 => en_us(),
        _ => en_us(),
    };
    let ping_res = controller::ping_controller(&i18n_items).await?;
    let real_delay_res = controller::real_delay_controller(ping_res, &i18n_items).await?;
    let download_res = controller::download_controller(real_delay_res, &i18n_items).await?;
    let mut cli_output_str = format!("{}\n", i18n_items.main_i18n.test_result);
    let mut file_output_str = String::from("IP,Ping,Real Delay,Speed (MB/s)\n");
    for res in download_res {
        let mb_s = res.speed / 1024f64 / 1024f64;
        let cli_line = format!(
            "{} {:15}, {} {:4}, {} {:4}, {} {:.5} MB/s\n",
            i18n_items.main_i18n.ip,
            res.ip,
            i18n_items.main_i18n.ping,
            res.ping.as_millis(),
            i18n_items.main_i18n.real_delay,
            res.real_delay.as_millis(),
            i18n_items.main_i18n.download_speed,
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
    let if_save_file = Confirm::new()
        .with_prompt(i18n_items.main_i18n.if_save_result)
        .interact()?;
    if if_save_file {
        let mut div = std::env::current_dir().expect(i18n_items.main_i18n.cannot_get_dir);
        div.push("result.csv");
        std::fs::write(div, file_output_str).expect(i18n_items.main_i18n.failed_to_write);
        println!("{}", i18n_items.main_i18n.result_saved);
        std::thread::sleep(std::time::Duration::from_secs(3));
    }
    Ok(())
}
