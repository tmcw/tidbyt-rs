pub mod month {

    pub const DAYS: usize = 35;

    pub fn get_month_days() {
        let datetime = now();
        let mut rows = Vec::new();
        let date = now().date_naive();
        // Learning from
        // https://github.com/erikh/saturn/blob/main/src/ui/layout.rs#L665
        let naive_date = chrono::NaiveDate::from_ymd_opt(
            date.year_ce().1 as i32,
            date.month0() + 1,
            (date - chrono::Duration::days(datetime.weekday().num_days_from_sunday().into()))
                .day0()
                + 1,
        )?;
        let zero_time = chrono::NaiveTime::from_hms_opt(0, 0, 0);
        let mut begin = chrono::NaiveDateTime::new(naive_date, zero_time);

        for x in 0..DAYS {
            if x % DAYS_IN_WEEK == 0 && x != 0 {
                last_row.push((Cell::from("".to_string()), 0));
                rows.push(
                    Row::new(last_row.iter().map(|x| x.0.clone()).collect::<Vec<Cell>>()).height({
                        let res = last_row.iter().map(|res| res.1).max().unwrap_or(4) as u16;
                        if res > 4 {
                            res
                        } else {
                            4
                        }
                    }),
                );
                rows.push(Row::new(
                    ["", "", "", "", "", "", "", "", ""].map(Cell::from),
                ));
                last_row = Vec::new();
                last_row.push((Cell::from("".to_string()), 0));
            }

            last_row.push(build_data(&mut lock, begin).await);
            begin += chrono::Duration::days(1);
        }

        rows
    }
}
