pub mod adjusted_color {
    use anyhow::Result;
    use chrono::Local;
    use palette::{Darken, IntoColor, Srgb};
    use raqote::{SolidSource, Source};
    use std::str::FromStr;
    use suncalc::Timestamp;

    pub fn get_sun_darkening() -> f64 {
        let lat = 40.692778;
        let lon = -73.990278;
        let sun_pos = suncalc::get_position(Timestamp(Local::now().timestamp()), lat, lon);

        if sun_pos.altitude < 0.0 {
            // If the sun is down, dark this a lot
            0.8
        } else {
            // Otherwise, slightly darken throughout the day.
            (90.0 - sun_pos.altitude) / 180.0
        }
    }

    pub fn adjusted_color_with_tint(hex: &str, tint: f64) -> Result<raqote::Source<'static>> {
        let mut color = Srgb::from_str(&hex)?.into_linear();

        color = color.darken(get_sun_darkening() + tint);

        let Srgb {
            standard: _,
            red,
            green,
            blue,
        } = color.into_color();

        color_to_source(red, green, blue)
    }

    pub fn adjusted_color(hex: &str) -> Result<raqote::Source<'static>> {
        adjusted_color_with_tint(hex, 0.0)
    }

    pub fn color_to_source(red: f64, green: f64, blue: f64) -> Result<raqote::Source<'static>> {
        Ok(Source::Solid(SolidSource::from_unpremultiplied_argb(
            255,
            (red * 255.0).floor() as u8,
            (green * 255.0).floor() as u8,
            (blue * 255.0).floor() as u8,
        )))
    }
}
