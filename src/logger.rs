use std::fs::OpenOptions;
use std::io::Write;

pub fn log_action(action: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("gpio_log.txt")
        .expect("Не вдалося відкрити файл логів");
    writeln!(file, "{}", action).expect("Не вдалося записати в лог");
}

pub fn read_logs() -> String {
    std::fs::read_to_string("gpio_log.txt").unwrap_or_else(|_| "Логи поки що відсутні".to_string())
}