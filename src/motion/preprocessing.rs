pub fn preprocess_image(data: Vec<u8>, width: usize, height: usize) -> Vec<u8> {
    // box_blur(data, width, height)
    data
}

fn box_blur(mut data: Vec<u8>, width: usize, height: usize) -> Vec<u8> {
    let original = data.clone();

    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            let idx = y * width + x;
            let sum = original[idx - width - 1] as u16
                + original[idx - width] as u16
                + original[idx - width + 1] as u16
                + original[idx - 1] as u16
                + original[idx] as u16
                + original[idx + 1] as u16
                + original[idx + width - 1] as u16
                + original[idx + width] as u16
                + original[idx + width + 1] as u16;
            data[idx] = (sum / 9) as u8;
        }
    }

    for x in 0..width {
        data[x] = original[x];
        data[(height - 1) * width + x] = original[(height - 1) * width + x];
    }
    for y in 0..height {
        data[y * width] = original[y * width];
        data[y * width + width - 1] = original[y * width + width - 1];
    }

    data
}
