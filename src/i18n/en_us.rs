use crate::i18n::interface::{DownloadControllerI18n, I18nItems, MainI18n, PingControllerI18n};

pub fn en_us<'a>() -> I18nItems<'a> {
    let download_controller_i18n = DownloadControllerI18n {
        testing: "Testing: ",
        total_before_num: "Download speed test will be done on",
        total_after_num: "IPs",
    };
    let ping_controller_i18n = PingControllerI18n {
        getting_ips_from_cloudflare: "Getting IP List from Cloudflare",
        getting_ips_from_cloudflare_success: "Success",
        getting_ips_from_cloudflare_failed: "Failed. Using internal IP List as fallback",
        prompt_part1: "Input test rounds (0 ≤ x ≤ ",
        prompt_part2: ") (each round will take 10 seconds to test ",
        prompt_part3: " IPs)",
        invalid_input: "Invalid Input",
        will_test_before_num: "Will ping",
        will_test_after_num: "IPs",
    };
    let main_i18n = MainI18n {
        test_result: "Test result:",
        ip: "IP:",
        ping: "Ping:",
        real_delay: "Real Delay:",
        download_speed: "Speed:",
        if_save_result: "Save results?",
        result_saved: "Results have been saved into result.csv",
        cannot_get_dir: "Failed to get the program's working directory",
        failed_to_write: "Failed to write to file",
    };
    let en_us_i18n_items = I18nItems {
        download_controller_i18n,
        ping_controller_i18n,
        real_delay_controller_i18n: "Will test real delay to get 10 avaliable IPs",
        main_i18n,
    };
    return en_us_i18n_items;
}
