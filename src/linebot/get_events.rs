use crate::linebot::get_class_id_to_class_name::get_class_id_to_class_name;
use ical;
use reqwest::{Client, Error};
use std::io::BufReader;

#[derive(Debug)]
pub struct Event {
    pub summary: Option<String>,
    pub description: Option<String>,
    pub deadline: Option<String>,
    pub category: Option<String>,
}

pub async fn get_events(url: String) -> Result<Vec<Event>, Error> {
    let client = Client::new();

    let text = client.get(url).send().await?.text().await?;

    let buf = BufReader::new(text.as_bytes());

    let ical_calendar = ical::IcalParser::new(buf).next().unwrap().ok().unwrap();
    let ical_events = ical_calendar.events;

    let events: Vec<_> = ical_events
        .iter()
        .map(|e| Event {
            summary: e.properties[1].value.clone(),
            description: e.properties[2].value.clone(),
            deadline: e.properties[7].value.clone(),
            category: search_class_name(
                e.properties[8]
                    .value
                    .clone()
                    .unwrap_or_else(|| "".to_string()),
            ),
        })
        .collect();

    Ok(events)
}

fn search_class_name(id: String) -> Option<String> {
    let syllabus = get_class_id_to_class_name().ok().unwrap();
    let class_id: Vec<&str> = id.split('_').collect();
    syllabus.get(class_id[0]).cloned()
}
