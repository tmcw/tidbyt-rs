/// Get AQI information from AirNow.
pub mod aqi {
    use crate::TextWidget;
    use anyhow::anyhow;
    use anyhow::{Context, Result};
    use cached::proc_macro::cached;
    use reqwest::header::USER_AGENT;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct Monitor {
        aqi: Vec<f64>,
        // conc: Vec<f64>,
        // parameterName: String,
        // parameterDisplayName: String,
        // concUnit: String,
    }

    #[derive(Debug, Deserialize)]
    struct Location {
        monitors: Vec<Monitor>,
        // coordinates: Vec<f64>,
        // siteName: String,
        // stationID: String,
        // fullAQSCode: String,
        // intlCode: String,
        // utcOffset: f64,
        // startTimeUTC: String,
        // endTimeUTC: String,
        // utcDateTimes: Vec<String>,
        // fileWrittenDateTime: String,
    }

    #[cached(time = 120, result = true)]
    pub async fn get_aqi() -> Result<TextWidget> {
        println!("Recomputing weather");

        // This is hardcoded to Brooklyn. To point
        // it somewhere else, you'll need to look up
        // a different weather station.
        let resp: Location = reqwest::Client::new()
            .get("https://an_gov_data.s3.amazonaws.com/Sites/360470118.json")
            .header(USER_AGENT, "tidbyt")
            .send()
            .await?
            .json::<Location>()
            .await?;

        let mon = resp.monitors[0]
            .aqi
            .last()
            .context("Could not get monitor")?;

        let aqi = *mon;

        let aqi_range = (aqi / 10.0).floor() as u64;
        if aqi > 100.0 {
            Ok(TextWidget {
                text: format!("{aqi_range} AQI"),
                color: String::from(match aqi_range {
                    0..=4 => "#92dd67",
                    5..=10 => "#ffb537",
                    _ => "#ff3838",
                }),
            })
        } else {
            Err(anyhow!("AQI Normal"))
        }
    }
}
