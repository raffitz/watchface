use std::string::String;

pub fn blockpix(s: String) -> (String, u16) {
    let mut result_top = String::new();
    let mut result_bot = String::new();

    for c in s.chars() {
        match c {
            '0' => {
                result_top.push_str("⡔⡄");
                result_bot.push_str("⠑⠁");
            }
            '1' => {
                result_top.push_str("⢴⠀");
                result_bot.push_str("⠚⠂");
            }
            '2' => {
                result_top.push_str("⠔⡄");
                result_bot.push_str("⠚⠂");
            }
            '3' => {
                result_top.push_str("⢒⠆");
                result_bot.push_str("⠒⠁");
            }
            '4' => {
                result_top.push_str("⣆⡆");
                result_bot.push_str("⠀⠃");
            }
            '5' => {
                result_top.push_str("⣖⠂");
                result_bot.push_str("⠒⠁");
            }
            '6' => {
                result_top.push_str("⣔⠂");
                result_bot.push_str("⠑⠁");
            }
            '7' => {
                result_top.push_str("⢒⠆");
                result_bot.push_str("⠘⠀");
            }
            '8' => {
                result_top.push_str("⢔⠄");
                result_bot.push_str("⠑⠁");
            }
            '9' => {
                result_top.push_str("⢔⡄");
                result_bot.push_str("⠐⠁");
            }
            ':' => {
                result_top.push_str("⠠⠀");
                result_bot.push_str("⠈⠀");
            }
            '-' => {
                result_top.push_str("⣀⡀");
                result_bot.push_str("⠀⠀");
            }
            ' ' => {
                result_top.push_str("⠀⠀");
                result_bot.push_str("⠀⠀");
            }
            _ => {
                result_top.push_str("⣶⡆");
                result_bot.push_str("⠛⠃");
            }
        }
    }
    (
        format!("{}\n{}", result_top, result_bot),
        result_top.chars().count() as u16,
    )
}

pub fn segmentpix(s: String) -> (String, u16) {
    let mut result_top = String::new();
    let mut result_mid = String::new();
    let mut result_bot = String::new();

    for c in s.chars() {
        match c {
            '0' => {
                result_top.push_str("⡔⠒⡄");
                result_mid.push_str("⡅⠀⡅");
                result_bot.push_str("⠑⠒⠁");
            }
            '1' => {
                result_top.push_str("⠀⠀⡄");
                result_mid.push_str("⠀⠀⡅");
                result_bot.push_str("⠀⠀⠁");
            }
            '2' => {
                result_top.push_str("⠐⠒⡄");
                result_mid.push_str("⡔⠒⠁");
                result_bot.push_str("⠑⠒⠀");
            }
            '3' => {
                result_top.push_str("⠐⠒⡄");
                result_mid.push_str("⠐⠒⡅");
                result_bot.push_str("⠐⠒⠁");
            }
            '4' => {
                result_top.push_str("⡄⠀⡄");
                result_mid.push_str("⠑⠒⡅");
                result_bot.push_str("⠀⠀⠁");
            }
            '5' => {
                result_top.push_str("⡔⠒⠀");
                result_mid.push_str("⠑⠒⡄");
                result_bot.push_str("⠐⠒⠁");
            }
            '6' => {
                result_top.push_str("⡔⠒⠀");
                result_mid.push_str("⡕⠒⡄");
                result_bot.push_str("⠑⠒⠁");
            }
            '7' => {
                result_top.push_str("⠐⠒⡄");
                result_mid.push_str("⠀⠀⡅");
                result_bot.push_str("⠀⠀⠁");
            }
            '8' => {
                result_top.push_str("⡔⠒⡄");
                result_mid.push_str("⡕⠒⡅");
                result_bot.push_str("⠑⠒⠁");
            }
            '9' => {
                result_top.push_str("⡔⠒⡄");
                result_mid.push_str("⠑⠒⡅");
                result_bot.push_str("⠀⠀⠁");
            }
            ':' => {
                result_top.push_str("⠀⡀⠀");
                result_mid.push_str("⠀⡀⠀");
                result_bot.push_str("⠀⠀⠀");
            }
            '-' => {
                result_top.push_str("⠀⠀⠀");
                result_mid.push_str("⠐⠒⠀");
                result_bot.push_str("⠀⠀⠀");
            }
            ' ' => {
                result_top.push_str("⠀⠀⠀");
                result_mid.push_str("⠀⠀⠀");
                result_bot.push_str("⠀⠀⠀");
            }
            _ => {
                result_top.push_str("⡔⡒⡄");
                result_mid.push_str("⡕⡒⡅");
                result_bot.push_str("⠑⠒⠁");
            }
        }
    }
    (
        format!("{}\n{}\n{}", result_top, result_mid, result_bot),
        result_top.chars().count() as u16,
    )
}
