use chrono::Local;
use std::thread;
use std::time::Duration;

fn read_file_trimmed(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path).map(|v| v.trim().to_string())
}

fn main() {
    const CAPACITY: &str = "/sys/class/power_supply/BAT0/capacity";
    const CHARGE_STATUS: &str = "/sys/class/power_supply/BAT0/status";

    loop {
        let battery_level = match read_file_trimmed(CAPACITY) {
            Ok(v) => v,
            Err(_) => "(Error Reading Battery Level)".to_owned(),
        };

        let now = Local::now();
        let formatted_date = now.format("%A, %d %B");
        let formatted_time = now.format("%H:%M:%S");

        let battery_icon = match read_file_trimmed(CHARGE_STATUS) {
            Ok(trimmed_status) => match trimmed_status.as_str() {
                "Discharging" => "󰁾",
                _ => "󰂅",
            },
            Err(_) => "(Charging Status Unknown)",
        };

        println!(
            "{} {} // {} // {} ",
            battery_icon, battery_level, formatted_date, formatted_time
        );

        thread::sleep(Duration::from_secs(1));
    }
}
