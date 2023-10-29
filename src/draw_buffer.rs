pub mod draw_buffer {
    use raqote::DrawTarget;

    pub fn get_rgba(dt: DrawTarget) -> Vec<u8> {
        // let width = dt.width() as u32;
        // let height = dt.height() as u32;
        let buf_v: Vec<u32> = dt.into_vec();
        let buf: &Vec<u32> = buf_v.as_ref();
        let mut output = Vec::with_capacity(buf.len() * 4);

        for pixel in buf {
            let a = (pixel >> 24) & 0xffu32;
            let mut r = (pixel >> 16) & 0xffu32;
            let mut g = (pixel >> 8) & 0xffu32;
            let mut b = (pixel >> 0) & 0xffu32;

            if a > 0u32 {
                r = r * 255u32 / a;
                g = g * 255u32 / a;
                b = b * 255u32 / a;
            }

            output.push(r as u8);
            output.push(g as u8);
            output.push(b as u8);
            output.push(a as u8);
        }

        output
    }
}
