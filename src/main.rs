mod gpio;
mod dht;
mod logger;

use warp::Filter;
use std::sync::{Arc, Mutex};
use gpio::GpioControl;
use dht::read_dht11;
use logger::log_action;

#[tokio::main]
async fn main() {
    let gpio = Arc::new(Mutex::new(GpioControl::new(17)));
    
    let turn_on = warp::path("on")
        .map({
            let gpio = gpio.clone();
            move || {
                gpio.lock().unwrap().turn_on();
                log_action("GPIO УВІМКНЕНО");
                "GPIO УВІМКНЕНО"
            }
        });
    
    let turn_off = warp::path("off")
        .map({
            let gpio = gpio.clone();
            move || {
                gpio.lock().unwrap().turn_off();
                log_action("GPIO ВИМКНЕНО");
                "GPIO ВИМКНЕНО"
            }
        });
    
    let dht_route = warp::path("dht").map(|| read_dht11());
    
    let logs_route = warp::path("logs")
        .map(|| logger::read_logs());
    
    let routes = turn_on.or(turn_off).or(dht_route).or(logs_route);
    
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}