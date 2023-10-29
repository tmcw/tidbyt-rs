pub mod weather {
    use anyhow::Result;
    use cached::proc_macro::cached;
    use reqwest::header::USER_AGENT;
    use serde::Deserialize;

    use crate::TextWidget;

    #[derive(Debug, Deserialize)]
    struct Elevation {
        unitCode: String,
        value: i32,
    }

    #[derive(Debug, Deserialize)]
    struct ProbabilityOfPrecipitation {
        unitCode: String,
        value: i32,
    }

    // TODO: remove all of the fields we won't use.
    #[derive(Debug, Deserialize)]
    struct Period {
        number: i32,
        name: String,
        startTime: String,
        endTime: String,
        isDaytime: bool,
        temperature: i32,
        temperatureUnit: String,
        temperatureTrend: Option<String>,
        probabilityOfPrecipitation: ProbabilityOfPrecipitation,
        dewpoint: Temperature,
        relativeHumidity: ProbabilityOfPrecipitation,
        windSpeed: String,
        windDirection: String,
        icon: String,
        shortForecast: String,
        detailedForecast: String,
    }

    #[derive(Debug, Deserialize)]
    struct Temperature {
        unitCode: String,
        value: f64,
    }

    #[derive(Debug, Deserialize)]
    struct Properties {
        updated: String,
        units: String,
        forecastGenerator: String,
        generatedAt: String,
        updateTime: String,
        #[serde(rename = "validTimes")]
        valid_times: String,
        elevation: Elevation,
        periods: Vec<Period>,
    }

    #[derive(Debug, Deserialize)]
    struct Geometry {
        #[serde(rename = "type")]
        geometry_type: String,
        coordinates: Vec<Vec<Vec<f64>>>,
    }

    #[derive(Debug, Deserialize)]
    struct Feature {
        #[serde(rename = "@context")]
        context: Vec<serde_json::Value>,
        #[serde(rename = "type")]
        feature_type: String,
        geometry: Geometry,
        properties: Properties,
    }

    // TODO: generate more than a string for weather.
    #[cached(time = 120, result = true)]
    pub async fn get_weather() -> Result<TextWidget> {
        let resp: Feature = reqwest::Client::new()
            .get("https://api.weather.gov/gridpoints/OKX/33,33/forecast/hourly")
            .header(USER_AGENT, "tidbyt")
            .send()
            .await?
            .json::<Feature>()
            .await?;
        let temp = resp.properties.periods[0].temperature.to_string();
        Ok(TextWidget {
            text: format!("{}°", temp),
            color: String::from("#fff"),
        })
    }
}
