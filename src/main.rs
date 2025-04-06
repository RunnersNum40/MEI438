use anyhow::Result;
use embedded_svc::wifi::{ClientConfiguration, Configuration};
use esp_camera_rs::Camera;
use esp_idf_hal::gpio;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    http::server::{
        Configuration as HttpConfig, EspHttpConnection, EspHttpServer, Method, Request,
    },
    nvs::EspDefaultNvsPartition,
    wifi::{AuthMethod, EspWifi},
};
use std::{
    sync::{Arc, Mutex},
    thread::sleep,
    time::{Duration, Instant},
};

mod secrets;
use secrets::{WIFI_PASSWORD, WIFI_SSID};

fn setup() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
}

fn connect_wifi(modem: esp_idf_hal::modem::Modem) -> Result<EspWifi<'static>> {
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;
    let mut wifi = EspWifi::new(modem, sys_loop, Some(nvs))?;

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: WIFI_SSID.try_into().unwrap(),
        password: WIFI_PASSWORD.try_into().unwrap(),
        auth_method: AuthMethod::WPA2Personal,
        ..Default::default()
    }))?;

    wifi.start()?;
    wifi.connect()?;

    let start = Instant::now();
    while !wifi.is_connected()? {
        if start.elapsed() > Duration::from_secs(10) {
            anyhow::bail!("Wi-Fi connection timeout");
        }
        sleep(Duration::from_millis(100));
    }

    let start = Instant::now();
    while !wifi.is_up()? {
        if start.elapsed() > Duration::from_secs(10) {
            anyhow::bail!("Wi-Fi up timeout");
        }
        sleep(Duration::from_millis(100));
    }

    Ok(wifi)
}

fn create_camera(pins: gpio::Pins) -> Result<Camera<'static>> {
    Ok(Camera::new(
        pins.gpio32,
        pins.gpio0,
        pins.gpio5,
        pins.gpio18,
        pins.gpio19,
        pins.gpio21,
        pins.gpio36,
        pins.gpio39,
        pins.gpio34,
        pins.gpio35,
        pins.gpio25,
        pins.gpio23,
        pins.gpio22,
        pins.gpio26,
        pins.gpio27,
        esp_idf_sys::camera::pixformat_t_PIXFORMAT_JPEG,
        esp_idf_sys::camera::framesize_t_FRAMESIZE_VGA,
    )?)
}

fn stream_handler(camera: Arc<Mutex<Camera>>, req: Request<&mut EspHttpConnection>) -> Result<()> {
    let mut res = req.into_response(
        200,
        Some("OK"),
        &[("Content-Type", "multipart/x-mixed-replace; boundary=frame")],
    )?;

    loop {
        let data = {
            let cam = camera.lock().unwrap();
            cam.get_framebuffer().map(|fb| fb.data().to_vec())
        };

        if let Some(data) = data {
            res.write(b"--frame\r\nContent-Type: image/jpeg\r\n")?;
            let header = format!("Content-Length: {}\r\n\r\n", data.len());
            res.write(header.as_bytes())?;
            res.write(&data)?;
            res.write(b"\r\n")?;
        } else {
            log::warn!("No frame received");
        }

        sleep(Duration::from_millis(100));
    }
}

fn main() -> Result<()> {
    setup();
    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    let camera = Arc::new(Mutex::new(create_camera(pins)?));
    let wifi = connect_wifi(peripherals.modem)?;

    let mut server = EspHttpServer::new(&HttpConfig::default())?;
    server.fn_handler("/stream", Method::Get, move |req| {
        stream_handler(camera.clone(), req)
    })?;

    log::info!(
        "Streaming at: http://{}/stream",
        wifi.sta_netif().get_ip_info()?.ip
    );

    loop {
        sleep(Duration::from_secs(1));
    }
}
