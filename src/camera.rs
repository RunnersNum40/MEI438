use anyhow::Result;
use esp_camera_rs::Camera;
use esp_idf_hal::gpio;
use esp_idf_sys::camera;

#[derive(Clone)]
pub struct Frame {
    pub data: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

pub struct CameraWrapper {
    camera: Camera<'static>,
    width: usize,
    height: usize,
}

impl CameraWrapper {
    pub fn new(pins: gpio::Pins) -> Result<Self> {
        let cam = Camera::new(
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
            camera::pixformat_t_PIXFORMAT_GRAYSCALE,
            camera::framesize_t_FRAMESIZE_128X128,
        )?;
        Ok(CameraWrapper {
            camera: cam,
            width: 128,
            height: 128,
        })
    }

    pub fn capture_frame(&mut self) -> Result<Frame> {
        let fb = self
            .camera
            .get_framebuffer()
            .ok_or_else(|| anyhow::anyhow!("Failed to get framebuffer"))?;

        Ok(Frame {
            data: fb.data().to_vec(),
            width: fb.width(),
            height: fb.height(),
        })
    }
}
