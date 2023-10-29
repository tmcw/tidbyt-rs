pub mod image {
    use anyhow::{Context, Result};
    use raqote::{DrawOptions, DrawTarget, Point, SolidSource, Source};

    struct OwnedImage {
        width: i32,
        height: i32,
        data: Vec<u32>,
    }

    impl OwnedImage {
        pub fn new(width: i32, height: i32, data: Vec<u32>) -> Self {
            Self {
                width,
                height,
                data,
            }
        }
        pub fn raqote_image(&self) -> raqote::Image {
            raqote::Image {
                width: self.width,
                height: self.height,
                data: &self.data,
            }
        }
    }

    /**
    Read a buffer of PNG data and parse it into an
    OwnedImage that we can render with raquote
    */
    fn read_image(buf: &[u8]) -> Result<OwnedImage> {
        let decoder = png::Decoder::new(&buf[..]);
        let mut reader = decoder.read_info().context("Could not get info")?;

        let mut email_icon_buffer = vec![0; reader.output_buffer_size()];
        reader
            .next_frame(&mut email_icon_buffer)
            .context("Could not get next frame")?;
        let header_info = reader.info();

        let mut u32_values = Vec::new();

        // Iterate through the bytes in chunks of 4 and combine them into u32 values
        for i in (0..email_icon_buffer.len()).step_by(4) {
            let mut u32_value = 0;
            for j in 0..4 {
                u32_value <<= 8; // Shift the existing bits to the left by 8 bits
                u32_value |= email_icon_buffer[i + j] as u32; // Add the next byte
            }
            u32_values.push(u32_value);
        }
        Ok(OwnedImage::new(
            header_info.width as i32,
            header_info.height as i32,
            u32_values,
        ))
    }

    fn draw_ascii(
        dt: &mut DrawTarget,
        text: &str,
        start: Point,
        color: &Source,
        color_darker: &Source,
        color_darkest: &Source,
    ) {
        let transparent = Source::Solid(SolidSource::from_unpremultiplied_argb(0, 0u8, 0u8, 0u8));
        for (dy, line) in text.lines().enumerate() {
            for (dx, c) in line.chars().enumerate() {
                dt.fill_rect(
                    start.x + dx as f32,
                    start.y + dy as f32,
                    1.,
                    1.,
                    match c {
                        '#' => color,
                        '-' => color_darker,
                        '*' => color_darkest,
                        _ => &transparent,
                    },
                    &DrawOptions::new(),
                )
            }
        }
    }
}
