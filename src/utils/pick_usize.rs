/// ## pick_usize
/// 让用户输入一个 usize 数值并返回
pub fn pick_usize(min: usize, max: usize, default: usize, msg: &str) -> usize {
    let mut input_text = String::new();
    println!("{} ({} ≤ x ≤ {}), 默认: {}", msg, min, max, default);
    match std::io::stdin().read_line(&mut input_text) {
        Ok(_) => {}
        Err(_) => input_text = default.to_string(),
    }
    let result = match input_text.trim().parse::<usize>() {
        Ok(t) => {
            if t < min {
                println!("小于最小值，已经设为最小值: {}", min);
                return min;
            }
            if t > max {
                println!("大于最大值，已经设为最大值: {}", max);
                return max;
            }
            return t;
        }
        Err(_) => default,
    };
    return result;
}
