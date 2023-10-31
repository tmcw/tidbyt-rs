pub mod strava {
    use anyhow::{Context, Result};
    use cached::proc_macro::cached;
    use chrono::{DateTime, Datelike, Duration, Local, NaiveDate, Utc};
    use reqwest::header::USER_AGENT;
    use serde::{Deserialize, Serialize};
    extern crate uom;
    use std::collections::{BTreeMap, HashMap};
    use std::path::PathBuf;
    use std::{env, fs};
    use uom::si::f32::*;
    use uom::si::length::{meter, mile};

    #[derive(Debug, Serialize, Deserialize)]
    struct TokenResponse {
        token_type: String,
        access_token: String,
        expires_at: u64,
        expires_in: u32,
        refresh_token: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Activity {
        distance: f32,
        #[serde(rename = "type")]
        sport_type: SportType,
        start_date: DateTime<Utc>,
        // activity_type_deprecated: ActivityType,
    }

    #[derive(Debug, Deserialize)]
    pub struct MetaAthlete {
        // The fields for MetaAthlete go here.
    }

    #[derive(Debug, Deserialize, PartialEq)]
    pub enum SportType {
        AlpineSki,
        BackcountrySki,
        Badminton,
        Canoeing,
        Crossfit,
        EBikeRide,
        Elliptical,
        EMountainBikeRide,
        Golf,
        GravelRide,
        Handcycle,
        HighIntensityIntervalTraining,
        Hike,
        IceSkate,
        InlineSkate,
        Kayaking,
        Kitesurf,
        MountainBikeRide,
        NordicSki,
        Pickleball,
        Pilates,
        Racquetball,
        Ride,
        RockClimbing,
        RollerSki,
        Rowing,
        Run,
        Sail,
        Skateboard,
        Snowboard,
        Snowshoe,
        Soccer,
        Squash,
        StairStepper,
        StandUpPaddling,
        Surfing,
        Swim,
        TableTennis,
        Tennis,
        TrailRun,
        Velomobile,
        VirtualRide,
        VirtualRow,
        VirtualRun,
        Walk,
        WeightTraining,
        Wheelchair,
        Windsurf,
        Workout,
        Yoga,
    }

    #[derive(Debug, Deserialize)]
    pub struct PolylineMap {
        // The fields for PolylineMap go here.
    }

    type Activities = Vec<Activity>;

    fn get_token_file_path() -> Result<PathBuf> {
        dirs::home_dir()
            .map(|mut path| {
                path.push(".strava_token.json");
                path
            })
            .context(".strava_token could not be found")
    }

    async fn fresh_token() -> Result<TokenResponse> {
        let client_id = env::var("STRAVA_CLIENT_ID").expect("Missing STRAVA_CLIENT_ID");
        let client_secret = env::var("STRAVA_CLIENT_SECRET").expect("Missing STRAVA_CLIENT_SECRET");

        let path = get_token_file_path()?;
        let file_content = fs::read_to_string(path)?;
        let last_token: TokenResponse = serde_json::from_str(&file_content)?;

        let mut params = HashMap::new();
        params.insert("grant_type", "refresh_token");
        params.insert("access_token", &last_token.access_token);
        params.insert("refresh_token", &last_token.refresh_token);
        params.insert("client_id", &client_id);
        params.insert("client_secret", &client_secret);

        let refresh_grant = reqwest::Client::new()
            .post("https://www.strava.com/oauth/token")
            .form(&params)
            .send()
            .await?
            .json::<TokenResponse>()
            .await?;

        let serialized_data = serde_json::to_string(&refresh_grant)?;
        let path = get_token_file_path()?;
        fs::write(path, serialized_data)?;

        Ok(refresh_grant)
    }

    #[derive(Debug, Clone)]
    struct TimeBucket {
        miles: f64,
    }

    fn month_days() -> BTreeMap<NaiveDate, TimeBucket> {
        let today = Utc::now().date_naive();

        let start_of_month = NaiveDate::from_ymd(today.year(), today.month(), 1);

        let mut bins: BTreeMap<NaiveDate, TimeBucket> = BTreeMap::new();
        bins
    }

    fn week_days() -> BTreeMap<NaiveDate, TimeBucket> {
        let today = Utc::now().date_naive();
        let days_since_monday = today.weekday().num_days_from_monday();

        let mut bins: BTreeMap<NaiveDate, TimeBucket> = BTreeMap::new();
        for i in 0..days_since_monday {
            bins.insert(today - Duration::days(i as i64), TimeBucket { miles: 0.0 });
        }
        bins
    }

    /// Get the number of unread threads in my inbox
    #[cached(time = 120, result = true)]
    pub async fn get_strava() -> Result<(Option<f64>, f64, Vec<u64>)> {
        let tokens = fresh_token().await?;
        let resp: Activities = reqwest::Client::new()
            .get("https://www.strava.com/api/v3/athlete/activities")
            .header(USER_AGENT, "tidbyt")
            .bearer_auth(tokens.access_token)
            .send()
            .await?
            .json::<Activities>()
            .await?;

        let mut bins = week_days();

        let mut miles_total = 0.0;

        for run in resp {
            if run.sport_type == SportType::Run {
                let length_in_miles = Length::new::<meter>(run.distance).get::<mile>() as f64;
                let local_start = run.start_date.with_timezone(&Local).date_naive();

                if let Some(bucket) = bins.get_mut(&local_start) {
                    bucket.miles += length_in_miles;
                    miles_total += length_in_miles;
                }
            }
        }

        let mut mi: Vec<u64> = Vec::new();

        for val in bins.values() {
            mi.push(val.miles.round() as u64);
        }

        let run_today = bins.first_entry().map(|entry| entry.get().miles);

        Ok((run_today, miles_total, mi))
    }
}
