use std::collections::BTreeMap;
use std::ops::Add;

use crate::data::activity;
use crate::data::processor::{ReportData, ReportWriter};
use crate::view::format_util::format_duration_hh_mm;
use chrono::Duration;
use nu_ansi_term::Style;

pub struct Writer {}

impl ReportWriter for Writer {
    fn process(&self, data: &ReportData) -> anyhow::Result<()> {
        show_activities(&data.activities)
    }
}

type ProjectMap<'a> = BTreeMap<&'a str, (Vec<&'a activity::Activity>, Duration)>;

pub fn show_activities<'a>(activities: &'a [&'a activity::Activity]) -> anyhow::Result<()> {
    let total = sum_duration(activities);
    print_bold(format!("Total: {}\n", format_duration_hh_mm(&total)).as_str());

    let project_map = create_project_map(activities);
    for (project, (activities, duration)) in &project_map {
        print!("\n\nProject ");
        print_bold(format!("{} {}\n\n", project, format_duration_hh_mm(&duration)).as_str());

        let description_map = group_activities_by_description(activities);
        for (description, activities) in &description_map {
            let description_duration = sum_duration(activities);
            print!(
                "{}, {}\n",
                format_duration_hh_mm(&description_duration),
                description
            );
        }
    }
    Ok(())
}

fn print_bold(msg: &str) {
    let style = Style::new().bold();
    print!(
        "{}{}{}",
        style.prefix().to_string(),
        msg,
        style.infix(Style::new()).to_string()
    );
}

fn create_project_map<'a>(activities: &'a [&'a activity::Activity]) -> ProjectMap {
    let mut project_map: ProjectMap = BTreeMap::new();

    for a in activities {
        project_map
            .entry(&a.project)
            .or_insert_with(|| (Vec::<&'a activity::Activity>::new(), Duration::seconds(0)))
            .0
            .push(a);
    }

    for (activities, duration) in project_map.values_mut() {
        *duration = sum_duration(activities);
    }

    project_map
}

fn sum_duration(activities: &[&activity::Activity]) -> Duration {
    let mut duration = Duration::seconds(0);

    for activity in activities {
        duration = duration.add(activity.get_duration());
    }

    duration
}

fn group_activities_by_description<'a>(
    activities: &'a [&'a activity::Activity],
) -> BTreeMap<&str, Vec<&'a activity::Activity>> {
    let mut activity_map: BTreeMap<&str, Vec<&'a activity::Activity>> = BTreeMap::new();

    for a in activities {
        activity_map.entry(&a.description).or_default().push(a);
    }

    activity_map
}
