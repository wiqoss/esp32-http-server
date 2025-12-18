mod roaming;
mod http_server;

use std::sync::atomic::AtomicBool;
use std::thread;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::prelude::Peripherals;
use esp_idf_svc::nvs::EspDefaultNvsPartition;

/// Is network initialized
pub static NETWORK_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Do ESP light
pub static DO_LIGHTING: AtomicBool = AtomicBool::new(true);

fn main() {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    log::info!("Started!");

    // Get system things
    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();

    // Start Wi-Fi
    thread::spawn(move || {
        roaming::start_roaming(
            peripherals.modem,
            sys_loop,
            nvs
        )
    });

    thread::Builder::new()
        .stack_size(10240)
        .spawn(move || {
            http_server::start()
        }).expect("Failed to spawn HTTP Server thread");

    // Get the LED pin
    let mut led = PinDriver::output(peripherals.pins.gpio2).unwrap();

    // Make a loop
    loop {
        if DO_LIGHTING.load(std::sync::atomic::Ordering::Relaxed) {
            // Enable and wait
            led.set_high().unwrap();
            thread::sleep(std::time::Duration::from_millis(33));

            // Disable and wait
            led.set_low().unwrap();
            thread::sleep(std::time::Duration::from_millis(150));
        } else {
            thread::sleep(std::time::Duration::from_millis(1000));
        }
    }
}
