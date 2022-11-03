use chrono::Duration;

pub fn get_past_date(date: i64) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    for i in 1..=date {
        let full_date = (chrono::offset::Local::now() - Duration::days(i)).to_string();
        let date = (&full_date[..10]).to_owned();
        res.push(date);
    }

    res
}
