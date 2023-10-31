pub mod email {
    use anyhow::{Context, Result};
    use cached::proc_macro::cached;
    use chrono::{DateTime, Utc};
    use jmap_client::client::{Client, Credentials};
    use jmap_client::mailbox::{query::Filter, Role};
    use serde::{Deserialize, Serialize};
    use std::env;
    use std::fs::File;
    use std::io::{BufReader, BufWriter};
    use std::path::PathBuf;

    #[derive(Debug, Serialize, Deserialize)]
    struct Record {
        pub timestamp: DateTime<Utc>,
        count: u64,
    }

    fn get_email_file_path() -> Result<PathBuf> {
        dirs::home_dir()
            .map(|mut path| {
                path.push(".email.json");
                path
            })
            .context("Could not get email path")
    }

    fn get_data() -> Result<Vec<Record>> {
        let path = get_email_file_path()?;
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let u: Vec<Record> = serde_json::from_reader(reader)?;
        Ok(u)
    }

    fn save_data(records: &Vec<Record>) -> Result<()> {
        let path = get_email_file_path()?;
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, records)?;
        Ok(())
    }

    /// Get the number of unread threads in my inbox
    #[cached(time = 120, result = true)]
    pub async fn get_email_count() -> Result<(u64, Vec<u64>)> {
        let jmap_token = env::var("JMAP_TOKEN").expect("Missing JMAP_TOKEN");
        let mut records = get_data().unwrap_or_default();

        let client = Client::new()
            .credentials(Credentials::bearer(jmap_token))
            .connect("https://api.fastmail.com/jmap/session")
            .await?;

        let inbox_id = client
            .mailbox_query(Filter::role(Role::Inbox).into(), None::<Vec<_>>)
            .await?
            .take_ids()
            .pop()
            .context("Could not get inbox id")?;

        let inbox = client
            .mailbox_get(&inbox_id, None::<Vec<_>>)
            .await?
            .context("Could not get inbox")?;

        let count = inbox.total_threads() as u64;

        let rec = Record {
            timestamp: Utc::now(),
            count,
        };
        records.push(rec);

        save_data(&records)?;

        let rec_chart = records
            .iter()
            .map(|rec| rec.count)
            .rev()
            .take(10)
            .rev()
            .collect();

        Ok((count, rec_chart))
    }
}
