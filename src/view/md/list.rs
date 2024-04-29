use std::collections::HashSet;

use chrono::DateTime;
use chrono::Duration;
use chrono::Local;
use chrono::NaiveDateTime;
use chrono::TimeZone;
use chrono::Utc;
use nu_ansi_term::Style;

use crate::conf;
use crate::data::activity;
use crate::data::processor::ListData;
use crate::data::processor::ListWriter;

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

    let projects: HashSet<String> = activities
        .iter()
        .map(|&activity| activity.project.clone())
        .collect();

    if projects.len() != 1 {
        return Err(anyhow::anyhow!("Several projects!"));
    }

    let mut last: Option<&activity::Activity> = None;
    for item in activities.iter() {
        print_date(item, last);
        print_row(item);
        last = Some(&item);
    }

    Ok(())
}

fn print_date(activity: &activity::Activity, last: Option<&activity::Activity>) {
    let new = match last {
        Some(act) => act.start.date() != activity.start.date(),
        None => true,
    };
    if new {
        let style = Style::new().bold();
        print!(
            "\n{}{}{}\n",
            style.prefix().to_string(),
            activity.start.date(),
            style.infix(Style::new()).to_string()
        );
    }
}

fn print_row(activity: &activity::Activity) {
    print!(
        "| {} | {} | {} | {} |\n",
        activity.description,
        get_start_time(activity),
        get_stop_time(activity),
        format_duration(&activity.get_duration())
    )
}

fn get_start_time(activity: &activity::Activity) -> String {
    to_utc(activity.start).format(conf::FORMAT_TIME).to_string()
}

fn get_stop_time(activity: &activity::Activity) -> String {
    return activity.end.map_or_else(
        || "-".to_string(),
        |end| {
            to_utc(end)
                .format(get_date_time_format(end.date() != activity.start.date()))
                .to_string()
        },
    );
}

fn get_date_time_format(date: bool) -> &'static str {
    if date {
        return conf::FORMAT_DATETIME;
    }
    conf::FORMAT_TIME
}

fn to_utc(time: NaiveDateTime) -> DateTime<Utc> {
    let local_dt: DateTime<Local> = Local.from_local_datetime(&time).unwrap();
    // let local_dt: DateTime<Local> = DateTime::from_utc(time, Local);
    return local_dt.with_timezone(&Utc);
}

fn format_duration(duration: &Duration) -> String {
    let hours = duration.num_hours();
    let minutes = duration.num_minutes() % 60;
    format!("{:02}:{:02}", hours, minutes)
}
