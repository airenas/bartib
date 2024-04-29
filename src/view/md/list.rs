use chrono::DateTime;
use chrono::Local;
use chrono::NaiveDateTime;
use chrono::TimeZone;
use chrono::Utc;

use crate::conf;
use crate::data::activity;
use crate::data::processor::ListData;
use crate::data::processor::ListWriter;
use crate::view::format_util;

pub struct Writer {}

impl ListWriter for Writer {
    fn process(&self, data: &ListData) -> anyhow::Result<()> {
        list_activities(&&data.activities)
    }
}

pub fn list_activities(activities: &[&activity::Activity]) -> anyhow::Result<()> {
    if activities.is_empty() {
        println!("No activity to display");
        return Ok(());
    }

    for item in activities.iter() {
        print_row(item);
    }

    Ok(())
}

fn print_row(activity: &activity::Activity) {
    print!(
        "| {} | {} | {} | {} |\n",
        activity.description,
        get_start_time(activity),
        get_stop_time(activity),
        format_util::format_duration(&activity.get_duration())
    )
}

fn get_start_time(activity: &activity::Activity) -> String {
    to_utc(activity.start).format(conf::FORMAT_TIME).to_string()
}

fn get_stop_time(activity: &activity::Activity) -> String {
    return activity.end.map_or_else(
        || "-".to_string(),
        |end| to_utc(end).format(conf::FORMAT_TIME).to_string(),
    );
}

fn to_utc(time: NaiveDateTime) -> DateTime<Utc> {
    let local_dt: DateTime<Local> = Local.from_local_datetime(&time).unwrap();
    // let local_dt: DateTime<Local> = DateTime::from_utc(time, Local);
    return local_dt.with_timezone(&Utc);
}
