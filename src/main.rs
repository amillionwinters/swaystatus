use chrono::Local;
use std::fs::File;
use std::io::Read;
use std::io::{Error, ErrorKind};
use std::thread::sleep;
use std::time::Duration;

fn read_file_trimmed<'a>(path: &str, buffer: &'a mut [u8]) -> Result<&'a str, Error> {
    let mut file = File::open(path)?;
    let bytes = file.read(buffer)?;
    let content =
        std::str::from_utf8(&buffer[..bytes]).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

    Ok(content.trim())
}

fn main() {
    const CAPACITY: &str = "/sys/class/power_supply/BAT0/capacity";
    const CHARGE_STATUS: &str = "/sys/class/power_supply/BAT0/status";
    const ONE_SECOND: Duration = Duration::new(1, 0);

    let mut level_buf = [0u8; 16];
    let mut status_buf = [0u8; 16];

    loop {
        let battery_level = match read_file_trimmed(CAPACITY, &mut level_buf) {
            Ok(v) => v,
            Err(_) => "(Error Reading Battery Level)",
        };

        let battery_icon = match read_file_trimmed(CHARGE_STATUS, &mut status_buf) {
            Ok(trimmed_status) => match trimmed_status {
                "Discharging" => "󰁾",
                _ => "󰂅",
            },
            Err(_) => "(Charging Status Unknown)",
        };

        let now = Local::now();
        let formatted_date = now.format("%A, %d %B");
        let formatted_time = now.format("%H:%M:%S");

        println!("{battery_icon} {battery_level}% // {formatted_date} // {formatted_time}");
        sleep(ONE_SECOND);
    }
}
