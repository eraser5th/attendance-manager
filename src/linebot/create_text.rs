use crate::linebot::get_events::Event;
use chrono::{DateTime, Duration, Local};

pub fn create_text(event: Event) -> String {
    let now: DateTime<Local> = Local::now();
    let time: Vec<char> = event.deadline.unwrap().chars().collect();
    let mut year = String::new();
    let mut month = String::new();
    let mut day = String::new();
    let mut hour = String::new();
    let mut minutes = String::new();
    let mut seconds = String::new();
    
    for i in 0..4 {
        year.push(time[i])
    }
    for i in 4..6 {
        month.push(time[i])
    }
    for i in 6..8 {
        day.push(time[i])
    }
    for i in 9..11 {
        hour.push(time[i])
    }
    for i in 11..13 {
        minutes.push(time[i])
    }
    for i in 13..15 {
        seconds.push(time[i])
    }
    let t = format!("{}-{}-{}T{}-{}-{} {}", year, month, day, hour, minutes, seconds, time[15]);
    let time = DateTime::parse_from_rfc3339(&t)
        .unwrap()
        .with_timezone(&Local);
    let duration: Duration = time - now;
    format!(
        "
{}
{}
{}
締切日: {} あと{}日",
        event.category.unwrap(),
        event.summary.unwrap_or_else(|| "".to_string()),
        event.description.unwrap_or_else(|| "".to_string()),
        time.format("%Y-%m-%d").to_string(),
        duration.num_days()
    )
}
