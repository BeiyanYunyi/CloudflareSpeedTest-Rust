pub struct DownloadControllerI18n<'a> {
    pub total_before_num: &'a str,
    pub total_after_num: &'a str,
    pub testing: &'a str,
}

pub struct PingControllerI18n<'a> {
    pub reading_custom_file: &'a str,
    pub reading_custom_file_error: &'a str,
    pub getting_ips_from_cloudflare: &'a str,
    pub getting_ips_from_cloudflare_success: &'a str,
    pub getting_ips_from_cloudflare_failed: &'a str,
    pub internal_or_online: &'a str,
    pub generating_ips: &'a str,
    pub prompt_part1: &'a str,
    pub prompt_part2: &'a str,
    pub prompt_part3: &'a str,
    pub invalid_input: &'a str,
    pub will_test_before_num: &'a str,
    pub will_test_after_num: &'a str,
}

pub struct MainI18n<'a> {
    pub test_result: &'a str,
    pub ip: &'a str,
    pub ping: &'a str,
    pub real_delay: &'a str,
    pub download_speed: &'a str,
    pub if_save_result: &'a str,
    pub result_saved: &'a str,
    pub cannot_get_dir: &'a str,
    pub failed_to_write: &'a str,
}

pub struct I18nItems<'a> {
    pub download_controller_i18n: DownloadControllerI18n<'a>,
    pub ping_controller_i18n: PingControllerI18n<'a>,
    pub real_delay_controller_i18n: &'a str,
    pub main_i18n: MainI18n<'a>,
}
