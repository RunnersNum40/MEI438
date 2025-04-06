use anyhow::Result;
use embedded_svc::wifi::{ClientConfiguration, Configuration};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::http::server::{
    Configuration as HttpConfig, EspHttpConnection, EspHttpServer, Method, Request,
};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    nvs::EspDefaultNvsPartition,
    wifi::{AuthMethod, EspWifi},
};
use std::time::Instant;
use std::{thread::sleep, time::Duration};

mod secrets;
use secrets::{WIFI_PASSWORD, WIFI_SSID};

fn setup() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
}

fn get_wifi_driver() -> Result<EspWifi<'static>> {
    let peripherals = Peripherals::take()?;
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let wifi_driver = EspWifi::new(peripherals.modem, sys_loop, Some(nvs))?;

    Ok(wifi_driver)
}

fn connect_to_wifi(timeout_secs: u64) -> Result<EspWifi<'static>> {
    let mut wifi_driver = get_wifi_driver()?;

    const CHECK_INTERVAL: Duration = Duration::from_millis(100);
    let timeout = Duration::from_secs(timeout_secs);
    let start = Instant::now();

    log::info!("Starting Wi-Fi driver...");
    wifi_driver.set_configuration(&Configuration::Client(ClientConfiguration::default()))?;
    wifi_driver.start()?;

    log::info!("Scanning for access points...");
    let ap_infos = wifi_driver.scan()?;
    let ap = ap_infos
        .into_iter()
        .find(|ap| ap.ssid == WIFI_SSID)
        .ok_or_else(|| anyhow::anyhow!("Access point \"{}\" not found during scan", WIFI_SSID))?;

    log::info!(
        "Found access point \"{}\" on channel {}",
        WIFI_SSID,
        ap.channel
    );

    log::info!("Connecting to \"{}\"...", WIFI_SSID);
    wifi_driver.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: WIFI_SSID.try_into().expect("Invalid SSID"),
        password: WIFI_PASSWORD.try_into().expect("Invalid password"),
        channel: Some(ap.channel),
        auth_method: AuthMethod::WPA2Personal,
        ..Default::default()
    }))?;

    wifi_driver.connect()?;
    log::info!("Waiting for DHCP lease...");

    while !wifi_driver.is_connected()? {
        if start.elapsed() > timeout {
            anyhow::bail!("Connection to \"{}\" timed out", WIFI_SSID);
        }
        sleep(CHECK_INTERVAL);
    }

    let ip_info = wifi_driver.sta_netif().get_ip_info()?;
    log::info!("Connected. IP info: {:?}", ip_info);

    Ok(wifi_driver)
}

fn response(request: Request<&mut EspHttpConnection>) -> Result<()> {
    log::debug!(
        "Responding to request\nMethod: {:?}, URI: {:?}",
        request.method(),
        request.uri()
    );

    request.into_ok_response()?.write(b"Hello from ESP32!")?;

    Ok(())
}

fn main() {
    setup();

    let wifi_driver = connect_to_wifi(10).unwrap();

    let mut server = EspHttpServer::new(&HttpConfig::default()).unwrap();

    server.fn_handler("/", Method::Get, response).unwrap();

    loop {
        log::info!(
            "IP info: {:?}",
            wifi_driver.sta_netif().get_ip_info().unwrap()
        );
        sleep(Duration::from_secs(10));
    }
}
