use crate::i18n::interface::{
  ChooseIPsI18n, DownloadControllerI18n, I18nItems, MainI18n, PingControllerI18n,
};

pub fn zh_cn<'a>() -> I18nItems<'a> {
  let download_controller_zh_cn = DownloadControllerI18n {
    testing: "正在测试：",
    total_before_num: "将对",
    total_after_num: "个 ip 进行下载速度测试",
  };
  let ping_controller_zh_cn = PingControllerI18n {
    reading_custom_file: "从自定义 IP 文件获取 IP",
    reading_custom_file_error: "无法读取文件",
    getting_ips_from_cloudflare: "正在从 Cloudflare 获取 IP 列表",
    getting_ips_from_cloudflare_success: "从 Cloudflare 获取 IP 列表成功",
    getting_ips_from_cloudflare_failed: "从 Cloudflare 获取 IP 列表失败",
    choose_ips: "选择 IP 列表",
    internal_or_online: "Cloudflare 的在线 IPv6 地址列表中绝大多数地址不可用，使用内置列表吗？",
    generating_ips: "正在随机生成待 ping 的 IP 列表",
    prompt_part1: "请输入测试轮数 (0 ≤ x ≤ ",
    prompt_part2: ") (每轮 ",
    prompt_part3: " 个，用时 10 秒，互不重复)",
    invalid_input: "输入不合法",
    will_test_before_num: "将对",
    will_test_after_num: "个 ip 进行 ping 测试",
  };
  let main_i18n = MainI18n {
    test_result: "测试结果:",
    ip: "IP:",
    ping: "Ping:",
    real_delay: "真实延迟:",
    download_speed: "速度:",
    if_save_result: "是否保存结果？",
    result_saved: "结果已保存至运行目录的 result.csv 下",
    cannot_get_dir: "无法获取程序运行目录",
    failed_to_write: "文件写入失败",
  };
  let choose_ips_i18n = ChooseIPsI18n {
    use_original_ips: "使用内置 IP 列表",
    use_tested_ips: "使用测试过的 IP 列表",
    use_online_ips: "从 Cloudflare 获取 IP 列表",
  };
  let zh_cn_i18n_items = I18nItems {
    download_controller_i18n: download_controller_zh_cn,
    ping_controller_i18n: ping_controller_zh_cn,
    real_delay_controller_i18n: "将进行真实延迟测试以获得 50 个可用 ip",
    choose_ips_i18n,
    main_i18n,
  };
  return zh_cn_i18n_items;
}
