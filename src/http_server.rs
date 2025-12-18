use esp_idf_svc::http::Method;
use esp_idf_svc::http::server::{Configuration, EspHttpServer};
use esp_idf_svc::io::Write;
use crate::{DO_LIGHTING, NETWORK_INITIALIZED};

pub fn start() {
    // Wait for network
    while !NETWORK_INITIALIZED.load(std::sync::atomic::Ordering::Relaxed) {}

    // Create the server
    let mut server = EspHttpServer::new(&Configuration::default()).expect("Failed to create server");

    server.fn_handler("/switch_light", Method::Get, |request| {
        // Reverse the current value
        DO_LIGHTING.store(
            // Reverse the current value
            !DO_LIGHTING.load(std::sync::atomic::Ordering::Relaxed),

            std::sync::atomic::Ordering::Relaxed
        );
        request.into_ok_response().unwrap().write_all(
            format!("<html><body><h1>Updated to {}</h1></html></body>", DO_LIGHTING.load(std::sync::atomic::Ordering::Relaxed))
                .into_bytes().
                as_ref()
        )
    }).unwrap();

    log::info!("HTTP Server started");

    loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}