pub mod uv {
    use anyhow::{anyhow, Context, Result};
    use cached::proc_macro::cached;
    use reqwest::header::USER_AGENT;
    use serde::{Deserialize, Serialize};

    use crate::TextWidget;

    #[derive(Debug, Serialize, Deserialize)]
    struct Data {
        ORDER: i32,
        ZIP: String,
        CITY: String,
        STATE: String,
        DATE_TIME: String,
        UV_VALUE: i32,
    }

    /// Get the peak UV value today
    #[cached(time = 3600, result = true)]
    pub async fn get_uv() -> Result<TextWidget> {
        // https://www.epa.gov/enviro/web-services#uvindex
        let resp: Vec<Data> = reqwest::Client::new()
            .get("https://data.epa.gov/efservice/getEnvirofactsUVHOURLY/ZIP/11201/json")
            .header(USER_AGENT, "tidbyt")
            .send()
            .await?
            .json::<Vec<Data>>()
            .await?;

        let uv = resp
            .iter()
            .map(|d| d.UV_VALUE)
            .max()
            .context("Could not get max UV")?;

        let uv_color = match uv {
            0..=4 => "#92dd67",
            5..=9 => "#ffb537",
            _ => "#ff3838",
        };

        if uv < 5 {
            Err(anyhow!("UV is fine"))
        } else {
            Ok(TextWidget {
                text: format!("{} UV", uv),
                color: String::from(uv_color),
            })
        }
    }
}
