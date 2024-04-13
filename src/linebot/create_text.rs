use crate::linebot::get_events::Event;
use chrono::{Local, DateTime, Duration};

pub fn create_text(event: Event) -> String {
    let now: DateTime<Local> = Local::now();
    let time = DateTime::parse_from_rfc3339(&event.deadline.unwrap()).ok().unwrap().with_timezone(&Local);
    let duration: Duration = time - now;
    format!("
{}
{}
{}
締切日: {} あと{}日"
, event.category.unwrap(), event.summary.unwrap_or_else(|| "".to_string()), event.description.unwrap_or_else(|| "".to_string()), time.format("%Y-%m-%d").to_string(), duration.num_days())
}
