use std::collections::HashMap;
use std::time::Duration;
use std::time::UNIX_EPOCH;

use chrono_humanize::Accuracy;
use chrono_humanize::HumanTime;
use chrono_humanize::Tense;
use rocket_dyn_templates::tera::{Value, to_value};

pub fn format_time_ago(epoch_secs: &Value, _: &HashMap<String, Value>) -> rocket_dyn_templates::tera::Result<Value> {
    if let Value::Object(n) = epoch_secs {

        println!("{:?}", n);
        if let Some(secs) = (match n.get("secs_since_epoch") {
            Some(v) => v,
            None => {return Err("Missing number".into())},
        }).as_u64() {
            return to_value(HumanTime::from(UNIX_EPOCH + Duration::from_secs(secs)).to_text_en(Accuracy::Rough, Tense::Past)).map_err(|_| rocket_dyn_templates::tera::Error::msg("Could not map tense"));
        }
    }

    Err("Invalid duration".into())
}

pub fn format_duration(secs: &Value, _: &HashMap<String, Value>) -> rocket_dyn_templates::tera::Result<Value> {
    if let Value::Object(n) = secs {

        println!("{:?}", n);
        if let Some(secs) = (match n.get("secs") {
            Some(v) => v,
            None => {return Err("Missing number".into())},
        }).as_u64() {
            return to_value(human_readable_duration(secs)).map_err(|_| rocket_dyn_templates::tera::Error::msg("Could not format duration"));
        }
    }

    Err("Invalid duration".into())
}
fn human_readable_duration(seconds: u64) -> String {
    let duration = Duration::from_secs(seconds);
    let mut formatted = String::new();

    let years = duration.as_secs() / (60 * 60 * 24 * 365);
    let months = (duration.as_secs() % (60 * 60 * 24 * 365)) / (60 * 60 * 24 * 30);
    let days = (duration.as_secs() % (60 * 60 * 24 * 30)) / (60 * 60 * 24);
    let hours = (duration.as_secs() % (60 * 60 * 24)) / (60 * 60);
    let minutes = (duration.as_secs() % (60 * 60)) / 60;
    let seconds = duration.as_secs() % 60;

    if years > 0 {
        formatted.push_str(&format!("{} years ", years));
    }
    if months > 0 {
        formatted.push_str(&format!("{} months ", months));
    }
    if days > 0 {
        formatted.push_str(&format!("{} days ", days));
    }
    if hours > 0 {
        formatted.push_str(&format!("{} hours ", hours));
    }
    if minutes > 0 {
        formatted.push_str(&format!("{} minutes ", minutes));
    }
    if seconds > 0 {
        formatted.push_str(&format!("{} seconds", seconds));
    }

    formatted.trim().to_string()
}
