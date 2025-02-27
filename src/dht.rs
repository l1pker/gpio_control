use dht_sensor::{dht11, Reading};
#[cfg(target_os = "linux")]
use rppal::gpio::Gpio;

const DHT_PIN: u8 = 4;

pub fn read_dht11() -> String {
    #[cfg(target_os = "linux")]
    {
        let gpio = Gpio::new().expect("Не вдалося отримати доступ до GPIO");
        let pin = gpio.get(DHT_PIN).unwrap().into_input_output();
        match dht11::read(&mut pin) {
            Ok(Reading { temperature, humidity }) => {
                let log_entry = format!("Температура: {}°C, Вологість: {}%", temperature, humidity);
                crate::logger::log_action(&log_entry);
                log_entry
            }
            Err(_) => {
                let error_msg = "Помилка зчитування DHT11".to_string();
                crate::logger::log_action(&error_msg);
                error_msg
            }
        }
    }
    
    #[cfg(not(target_os = "linux"))]
    "Емуляція: Температура 25°C, Вологість 50%".to_string()
}